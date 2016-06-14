// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use regex::Regex;

use hcore::fs;
use util;

#[test]
#[ignore]
fn builds_a_service() {

    let mut simple_service = match util::command::plan_build(&util::path::fixture_as_string("bldr_build")) {
        Ok(cmd) => cmd,
        Err(e) => panic!("{:?}", e),
    };

    simple_service.wait_with_output();
    assert_cmd_exit_code!(simple_service, [0]);
    assert_regex!(simple_service.stdout(), r"Loading /.*/plan.sh");
    assert_regex!(simple_service.stdout(),
                  &format!(r"{}/bldr_build-0.0.1",
                           fs::cache_src_path(None).to_string_lossy()));
    assert_regex!(simple_service.stdout(),
                  &format!(r"/{}/test/bldr_build/0.0.1/\d{{14}}", fs::PKG_PATH));
    assert_regex!(simple_service.stdout(),
                  &format!(r"{}/test-bldr_build-0.0.1-\d{{14}}.bldr",
                           fs::cache_artifact_path(None).to_string_lossy()));
    let pkg_re = Regex::new(&format!(r"({}/test-bldr_build-0.0.1-\d{{14}}.bldr)",
                                     fs::cache_artifact_path(None).to_string_lossy()))
        .unwrap();
    let caps = pkg_re.captures(simple_service.stdout()).unwrap();
    if let Some(pkg_path) = caps.at(1) {
        assert_file_exists_in_studio!(pkg_path);
    }
}
