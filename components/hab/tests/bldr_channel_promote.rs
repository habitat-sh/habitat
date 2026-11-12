use std::{io::{Read,
               Write},
          net::TcpListener,
          process::{Command,
                    Output},
          thread,
          time::{Duration,
                 Instant}};

struct PromoteRun {
    output:  Output,
    request: String,
}

fn run_promotion(flags: &[&str], status: &str, body: &str) -> PromoteRun {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let response = format!("HTTP/1.1 {status}\r\nContent-Type: \
                            application/json\r\nContent-Length: {}\r\nConnection: \
                            close\r\n\r\n{body}",
                           body.len());
    let server = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream.set_read_timeout(Some(Duration::from_secs(5)))
                          .unwrap();
                    let mut request = Vec::new();
                    let mut buffer = [0_u8; 4096];
                    while !request.ends_with(b"\r\n\r\n") {
                        let count = stream.read(&mut buffer).unwrap();
                        assert!(count > 0, "Builder request ended before headers");
                        request.extend_from_slice(&buffer[..count]);
                    }
                    stream.write_all(response.as_bytes()).unwrap();
                    return String::from_utf8(request).unwrap();
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline,
                            "hab did not contact the mock Builder");
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("mock Builder failed: {error}"),
            }
        }
    });

    let mut command = Command::new(env!("CARGO_BIN_EXE_hab"));
    command.args(["bldr", "channel", "promote"])
           .args(flags)
           .args(["dev", "stable", "-o", "core", "-z", "secret", "--url"])
           .arg(format!("http://{address}"))
           .env("HAB_LICENSE", "accept-no-persist")
           .env("HAB_NOCOLORING", "1")
           .env("HAB_NONINTERACTIVE", "1");
    let output = command.output().unwrap();
    PromoteRun { output,
                 request: server.join().unwrap() }
}

fn stdout(run: &PromoteRun) -> String { String::from_utf8(run.output.stdout.clone()).unwrap() }

fn stderr(run: &PromoteRun) -> String { String::from_utf8(run.output.stderr.clone()).unwrap() }

#[test]
fn compatibility_conflict_exits_nonzero_and_explains_the_conflict() {
    let body = r#"{"error":"compatibility_check_failed","conflicts":{"x86_64-linux":{"core/openssl":["core/openssl/1/1","core/openssl/2/1"]}},"conflict_within_source":true,"details":{"x86_64-linux":{"core/openssl":{"summary":"openssl has 2 versions","idents":[{"ident":"core/openssl/1/1","introduced_by":{"source":{"total_count":1,"examples":[{"package":"core/app/1/1","relation":"tdep"}]},"target_only":{"total_count":0,"truncated":false,"packages":[]}}},{"ident":"core/openssl/2/1","introduced_by":{"source":{"total_count":0,"examples":[]},"target_only":{"total_count":1,"truncated":false,"packages":[{"package":"core/other/1/1","relation":"head"}]}}}]}}}}"#;
    let run = run_promotion(&["--check"], "409 Conflict", body);
    assert!(!run.output.status.success());
    assert!(run.request
               .starts_with("PUT /v1/depot/channels/core/dev/pkgs/promote?channel=stable&\
                             check=true "));
    let output = stdout(&run);
    for expected in ["Compatibility check failed",
                     "Target x86_64-linux",
                     "core/openssl/1/1",
                     "core/openssl/2/1",
                     "source: core/app/1/1",
                     "target: core/other/1/1",
                     "source channel also contains"]
    {
        assert!(output.contains(expected), "missing {expected}: {output}");
    }
    assert!(stderr(&run).contains("no packages were promoted"));
}

#[test]
fn conflict_json_is_raw_and_has_no_status_text() {
    let body = r#"{"error":"compatibility_check_failed","conflicts":{"x86_64-linux":{"core/openssl":["core/openssl/1/1","core/openssl/2/1"]}},"conflict_within_source":false,"details":{"x86_64-linux":{"core/openssl":{"summary":"openssl has 2 versions","idents":[]}}}}"#;
    let run = run_promotion(&["--check", "--json"], "409 Conflict", body);
    assert!(!run.output.status.success());
    assert_eq!(stdout(&run).trim(), body);
    assert!(!stderr(&run).contains("Promoting"));
    assert!(!stdout(&run).contains("Promoting"));
}

#[test]
fn snapshot_success_prints_name_and_packages() {
    let body = r#"{"snapshot_channel":"stable_SS_example","packages":{"x86_64-linux":{"core":{"app":[{"ident":"core/app/1/1","origin":"core","name":"app","version":"1","release":"1"}],"openssl":[{"ident":"core/openssl/3/1","origin":"core","name":"openssl","version":"3","release":"1"}]}}}}"#;
    let run = run_promotion(&["--check", "--snapshot"], "200 OK", body);
    assert!(run.output.status.success(), "{}", stderr(&run));
    assert!(run.request
               .contains("channel=stable&check=true&snapshot=true"));
    let output = stdout(&run);
    for expected in ["Compatibility check passed",
                     "Snapshot channel: stable_SS_example",
                     "Target x86_64-linux",
                     "core/app/1/1",
                     "core/openssl/3/1"]
    {
        assert!(output.contains(expected), "missing {expected}: {output}");
    }
}

#[test]
fn snapshot_json_contains_only_the_builder_response() {
    let body = r#"{"snapshot_channel":"stable_SS_example","packages":{"x86_64-linux":{"core":{"app":[{"ident":"core/app/1/1","origin":"core","name":"app","version":"1","release":"1"}]}}}}"#;
    let run = run_promotion(&["--snapshot", "--json"], "200 OK", body);
    assert!(run.output.status.success(), "{}", stderr(&run));
    let output = stdout(&run);
    let json: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
    assert_eq!(json["snapshot_channel"], "stable_SS_example");
    assert_eq!(json["packages"]["x86_64-linux"]["core"]["app"][0]["ident"],
               "core/app/1/1");
    assert!(!output.contains("Promoting"));
    assert!(!stderr(&run).contains("Promoting"));
}

#[test]
fn empty_legacy_response_warns_about_both_unconfirmed_flags() {
    let run = run_promotion(&["--check", "--snapshot"], "200 OK", "");
    assert!(run.output.status.success(), "{}", stderr(&run));
    assert!(run.request
               .contains("channel=stable&check=true&snapshot=true"));
    let warnings = stderr(&run);
    assert!(warnings.contains("no confirmation that the compatibility check ran"),
            "{warnings}");
    assert!(warnings.contains("No snapshot was created"), "{warnings}");
    assert!(!stdout(&run).contains("Compatibility check passed"));
    assert!(!stdout(&run).contains("Snapshot channel:"));
}
