// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use regex::Regex;

use util;
use setup;

#[test]
fn builds_a_service() {
    setup::gpg_import();

    let mut simple_service =
        match util::command::bldr_build(&util::path::fixture_as_string("bldr_build")) {
            Ok(cmd) => cmd,
            Err(e) => panic!("{:?}", e),
        };

    simple_service.wait_with_output();
    assert_cmd_exit_code!(simple_service, [0]);
    assert_regex!(simple_service.stdout(), r"Loading /.*/plan.sh");
    assert_regex!(simple_service.stdout(),
                  r"/opt/bldr/cache/src/bldr_build-0.0.1");
    assert_regex!(simple_service.stdout(),
                  r"/opt/bldr/pkgs/test/bldr_build/0.0.1/\d{14}");
    assert_regex!(simple_service.stdout(),
                  r"/opt/bldr/cache/pkgs/test-bldr_build-0.0.1-\d{14}.bldr");
    let pkg_re = Regex::new(r"(/opt/bldr/cache/pkgs/test-bldr_build-0.0.1-\d{14}.bldr)").unwrap();
    let caps = pkg_re.captures(simple_service.stdout()).unwrap();
    if let Some(pkg_path) = caps.at(1) {
        assert_file_exists_in_studio!(pkg_path);
    }
}
