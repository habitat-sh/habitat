//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::process::Command;

use regex::Regex;
use tempdir::TempDir;

use util;
use setup;

#[test]
fn builds_a_service() {
    setup::gpg_import();

    let tempdir = TempDir::new("simple_service").unwrap();
    let mut copy_cmd = Command::new("cp")
        .arg("-r")
        .arg(util::path::fixture("bldr_build"))
        .arg(tempdir.path().to_str().unwrap())
        .spawn().unwrap();
    copy_cmd.wait().unwrap();

    let mut simple_service = match util::command::bldr_build(tempdir.path().join("bldr_build")) {
        Ok(cmd) => cmd,
        Err(e) => panic!("{:?}", e)
    };

    simple_service.wait_with_output();
    assert_cmd_exit_code!(simple_service, [0]);
    assert_regex!(simple_service.stdout(), r"Loading /.*/Bldrfile");
    assert_regex!(simple_service.stdout(), r"bldr_build: Cache: /opt/bldr/cache/src/bldr_build-0.0.1");
    assert_regex!(simple_service.stdout(), r"bldr_build: Installed: /opt/bldr/pkgs/test/bldr_build/0.0.1/\d{14}");
    assert_regex!(simple_service.stdout(), r"bldr_build: Package: /opt/bldr/cache/pkgs/test-bldr_build-0.0.1-\d{14}.bldr");
    let pkg_re = Regex::new(r"bldr_build: Package: (/opt/bldr/cache/pkgs/test-bldr_build-0.0.1-\d{14}.bldr)").unwrap();
    let caps = pkg_re.captures(simple_service.stdout()).unwrap();
    if let Some(pkg_path) = caps.at(1) {
        assert_file_exists!(pkg_path);
    }
}
