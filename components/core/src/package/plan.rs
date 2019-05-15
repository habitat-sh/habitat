use crate::error::{Error,
                   Result};
use serde_derive::{Deserialize,
                   Serialize};
use std::io::BufRead;

#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    pub name:    String,
    pub origin:  String,
    pub version: Option<String>,
}

impl Plan {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut name: Option<String> = None;
        let mut origin: Option<String> = None;
        let mut version: Option<String> = None;
        for line in bytes.lines() {
            if let Ok(line) = line {
                // Rather than just blindly accepting values, let's trim all the
                // whitespace first, verify that we actually have 2 things separated
                // by an equal sign, and strip out quotes of any kind.
                //
                // To do this properly, we probably need some kind of bash parser,
                // or a plan file syntax that's in a different language that we do
                // have a parser for (LUA!), but both of those things are beyond the
                // scope of this task.
                let parts: Vec<&str> = line.splitn(2, '=').map(str::trim).collect();

                if parts.len() != 2 {
                    continue;
                }

                let mut val = parts[1].replace("\"", "");
                val = val.replace("'", "");

                match parts[0] {
                    "pkg_name" | "$pkg_name" => name = Some(val),
                    "pkg_origin" | "$pkg_origin" => origin = Some(val),
                    "pkg_version" | "$pkg_version" => version = Some(val),
                    _ => (),
                }
            }
        }

        if name.is_none() || origin.is_none() {
            return Err(Error::PlanMalformed);
        }

        Ok(Plan { name: name.unwrap(),
                  origin: origin.unwrap(),
                  version })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_plan_with_no_quotes_works() {
        let content = r#"
        pkg_origin=neurosis
        pkg_name=testapp
        pkg_version=0.1.3
        pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
        pkg_license=()
        pkg_upstream_url=https://github.com/habitat-sh/habitat-example-plans
        pkg_source=nosuchfile.tar.gz
        pkg_deps=()
        pkg_expose=()

        do_download() {
          return 0
        }

        do_verify() {
          return 0
        }

        do_unpack() {
          return 0
        }

        do_build() {
          return 0
        }

        do_install() {
          return 0
        }
        "#;
        let plan = Plan::from_bytes(content.as_bytes()).unwrap();
        assert_eq!(plan.origin, "neurosis".to_string());
        assert_eq!(plan.name, "testapp".to_string());
        assert_eq!(plan.version, Some("0.1.3".to_string()));
    }

    #[test]
    fn parsing_plan_with_double_quotes_works() {
        let content = r#"
        pkg_origin="neurosis"
        pkg_name="testapp"
        pkg_version="0.1.3"
        pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
        pkg_license=()
        pkg_upstream_url=https://github.com/habitat-sh/habitat-example-plans
        pkg_source=nosuchfile.tar.gz
        pkg_deps=()
        pkg_expose=()

        do_download() {
          return 0
        }

        do_verify() {
          return 0
        }

        do_unpack() {
          return 0
        }

        do_build() {
          return 0
        }

        do_install() {
          return 0
        }
        "#;
        let plan = Plan::from_bytes(content.as_bytes()).unwrap();
        assert_eq!(plan.origin, "neurosis".to_string());
        assert_eq!(plan.name, "testapp".to_string());
        assert_eq!(plan.version, Some("0.1.3".to_string()));
    }

    #[test]
    fn parsing_plan_with_single_quotes_works() {
        let content = r#"
        pkg_origin='neurosis'
        pkg_name='testapp'
        pkg_version='0.1.3'
        pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
        pkg_license=()
        pkg_upstream_url=https://github.com/habitat-sh/habitat-example-plans
        pkg_source=nosuchfile.tar.gz
        pkg_deps=()
        pkg_expose=()

        do_download() {
          return 0
        }

        do_verify() {
          return 0
        }

        do_unpack() {
          return 0
        }

        do_build() {
          return 0
        }

        do_install() {
          return 0
        }
        "#;
        let plan = Plan::from_bytes(content.as_bytes()).unwrap();
        assert_eq!(plan.origin, "neurosis".to_string());
        assert_eq!(plan.name, "testapp".to_string());
        assert_eq!(plan.version, Some("0.1.3".to_string()));
    }

    #[test]
    fn parsing_windows_plan_works() {
        let content = r#"
        $pkg_name="testapp"
        $pkg_origin="neurosis"
        $pkg_version="1.04"
        $pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"

        function Invoke-Unpack {
        }

        function Invoke-Install {
        }
        "#;
        let plan = Plan::from_bytes(content.as_bytes()).unwrap();
        assert_eq!(plan.origin, "neurosis".to_string());
        assert_eq!(plan.name, "testapp".to_string());
        assert_eq!(plan.version, Some("1.04".to_string()));
    }
}
