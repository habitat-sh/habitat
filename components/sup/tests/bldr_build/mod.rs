// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use regex::Regex;

use hcore::fs;
use util;
use setup;

#[test]
fn builds_a_service() {
    setup::gpg_import();

    let mut simple_service = match util::command::plan_build(&util::path::fixture_as_string("bldr_build")) {
        Ok(cmd) => cmd,
        Err(e) => panic!("{:?}", e),
    };

    simple_service.wait_with_output();
    assert_cmd_exit_code!(simple_service, [0]);
    assert_regex!(simple_service.stdout(), r"Loading /.*/plan.sh");
    assert_regex!(simple_service.stdout(),
                  &format!(r"{}/bldr_build-0.0.1", fs::CACHE_SRC_PATH));
    assert_regex!(simple_service.stdout(),
                  &format!(r"{}/test/bldr_build/0.0.1/\d{{14}}", fs::PKG_PATH));
    assert_regex!(simple_service.stdout(),
                  &format!(r"{}/test-bldr_build-0.0.1-\d{{14}}.bldr",
                           fs::CACHE_ARTIFACT_PATH));
    let pkg_re = Regex::new(&format!(r"({}/test-bldr_build-0.0.1-\d{{14}}.bldr)",
                                     fs::CACHE_ARTIFACT_PATH))
                     .unwrap();
    let caps = pkg_re.captures(simple_service.stdout()).unwrap();
    if let Some(pkg_path) = caps.at(1) {
        assert_file_exists_in_studio!(pkg_path);
    }
}
