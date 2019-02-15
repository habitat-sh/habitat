// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

//! Defines the data that we provide users in their template files.
//!
//! The data structures in this module effectively serve as wrappers
//! or proxies for other Supervisor-internal data structures. They use
//! `Cow` types for flexibility; in the normal code flow, they will
//! just take references to existing data, keeping the memory
//! footprint low. For tests, however, they can be created directly
//! with test data, which means you don't need to instantiate a lot of
//! complex data structures just to verify their behavior.
//!
//! Using custom data type proxies like this allows us to decouple the
//! internal data structures from the needs of the templating
//! engine. Since the ultimate purpose of the rendering context is to
//! create a JSON object, we can specify our own custom `Serialize`
//! implementations, completely separate from any implementations on
//! the original data structures. This allows us to further decouple
//! things, giving us the ability to add new fields to the rendering
//! context at the serialization level (e.g., to add the same data
//! under a different name, introduce new views on existing data,
//! etc.), which finally gives us a safe mechanism by which to evolve
//! our rendering context.
//!
//! As such, know that any changes to the `Serialize` implementations
//! in this module will have an immediate and direct effect on the
//! data that is available in users' templates. Make changes with care
//! and deliberation.
//!
//! To help guard against this, the entire structure of the rendering
//! context is also defined in a JSON Schema document, which is used
//! in tests to validate everything.
//!
//! All proxy types and implementations are private, to emphasize
//! their focused and single-use purpose; they shouldn't be used for
//! anything else, and so, they _can't_ be used for anything else.

use std::{borrow::Cow, collections::HashMap, path::PathBuf, result};

use serde::{ser::SerializeMap, Serialize, Serializer};

use super::{
    config::Cfg,
    package::{Env, Pkg},
};
use crate::hcore::package::PackageIdent;

/// The context of a rendering call, exposing information on the
/// currently-running Supervisor and service, its service group, and
/// groups it is bound to. The JSON serialization of this
/// structure is what is exposed to users in their templates.
///
/// NOTE: This public interface of this structure is defined by its
/// Serde `Serialize` implementation (and those of its members), so
/// change this with care.
///
/// User-facing documentation is available at
/// https://www.habitat.sh/docs/reference/#template-data; update that
/// as required.
#[derive(Clone, Debug, Serialize)]
pub struct RenderContext<'a> {
    pkg: Package<'a>,
    cfg: Cow<'a, Cfg>,
}

impl<'a> RenderContext<'a> {
    /// Create a RenderContext that wraps a number of internal data
    /// structures, safely and selectively exposing the data to users
    /// in their templates.
    ///
    /// Note that we wrap everything except the `Cfg`, to which we
    /// maintain a direct reference. The serialization logic for this
    /// is already complex, and exactly what we need. Because of the
    /// nature of `Cfg`s behavior, we should be safe relying on that
    /// implementation for the foreseeable future.
    pub fn new(pkg: &'a Pkg, cfg: &'a Cfg) -> RenderContext<'a> {
        RenderContext {
            pkg: Package::from_pkg(pkg),
            cfg: Cow::Borrowed(cfg),
        }
    }
}

////////////////////////////////////////////////////////////////////////
// PRIVATE CODE BELOW
////////////////////////////////////////////////////////////////////////

/// Templating proxy fro a `manager::service::Pkg` struct.
///
/// Currently exposed to users under the `pkg` key.
#[derive(Clone, Debug)]
struct Package<'a> {
    ident: Cow<'a, PackageIdent>,
    origin: Cow<'a, String>,
    name: Cow<'a, String>,
    version: Cow<'a, String>,
    release: Cow<'a, String>,
    deps: Cow<'a, Vec<PackageIdent>>,
    env: Cow<'a, Env>,
    // TODO (CM): Ideally, this would be Vec<u16>, since they're ports.
    exposes: Cow<'a, Vec<String>>,
    exports: Cow<'a, HashMap<String, String>>,
    // TODO (CM): Maybe Path instead of Cow<'a PathBuf>?
    path: Cow<'a, PathBuf>,
    svc_path: Cow<'a, PathBuf>,
    svc_config_path: Cow<'a, PathBuf>,
    svc_config_install_path: Cow<'a, PathBuf>,
    svc_data_path: Cow<'a, PathBuf>,
    svc_files_path: Cow<'a, PathBuf>,
    svc_static_path: Cow<'a, PathBuf>,
    svc_var_path: Cow<'a, PathBuf>,
    svc_pid_file: Cow<'a, PathBuf>,
    svc_run: Cow<'a, PathBuf>,
    svc_user: Cow<'a, String>,
    svc_group: Cow<'a, String>,
}

impl<'a> Package<'a> {
    fn from_pkg(pkg: &'a Pkg) -> Self {
        Package {
            ident: Cow::Borrowed(&pkg.ident),
            // TODO (CM): have Pkg use FullyQualifiedPackageIdent, and
            // get origin, name, version, and release from it, rather
            // than storing each individually; I suspect that was just
            // for templating
            origin: Cow::Borrowed(&pkg.origin),
            name: Cow::Borrowed(&pkg.name),
            version: Cow::Borrowed(&pkg.version),
            release: Cow::Borrowed(&pkg.release),
            deps: Cow::Borrowed(&pkg.deps),
            env: Cow::Borrowed(&pkg.env),
            exposes: Cow::Borrowed(&pkg.exposes),
            exports: Cow::Borrowed(&pkg.exports),
            path: Cow::Borrowed(&pkg.path),
            svc_path: Cow::Borrowed(&pkg.svc_path),
            svc_config_path: Cow::Borrowed(&pkg.svc_config_path),
            svc_config_install_path: Cow::Borrowed(&pkg.svc_config_install_path),
            svc_data_path: Cow::Borrowed(&pkg.svc_data_path),
            svc_files_path: Cow::Borrowed(&pkg.svc_files_path),
            svc_static_path: Cow::Borrowed(&pkg.svc_static_path),
            svc_var_path: Cow::Borrowed(&pkg.svc_var_path),
            svc_pid_file: Cow::Borrowed(&pkg.svc_pid_file),
            svc_run: Cow::Borrowed(&pkg.svc_run),
            svc_user: Cow::Borrowed(&pkg.svc_user),
            svc_group: Cow::Borrowed(&pkg.svc_group),
        }
    }
}

impl<'a> Serialize for Package<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Explicitly focusing on JSON serialization, which does not
        // need a length hint (thus the `None`)
        let mut map = serializer.serialize_map(None)?;

        // This is really the only thing that we need to have a custom
        // `Serialize` implementation for. Alternatively, we could
        // wrap our ident in a proxy type that has its own Serialize
        // implementation, but I think we're going to have some other
        // changes in this serialization format soon (e.g., around
        // `deps` and `exposes`, and eventually storing a
        // FullyQualifiedPackageIdent here).
        map.serialize_entry("ident", &self.ident.to_string())?;

        // Break out the components of the identifier, for easy access
        // in templates
        map.serialize_entry("origin", &self.origin)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("release", &self.release)?;

        map.serialize_entry("deps", &self.deps)?;
        map.serialize_entry("env", &self.env)?;

        map.serialize_entry("exposes", &self.exposes)?;
        map.serialize_entry("exports", &self.exports)?;
        map.serialize_entry("path", &self.path)?;

        map.serialize_entry("svc_path", &self.svc_path)?;
        map.serialize_entry("svc_config_path", &self.svc_config_path)?;
        map.serialize_entry("svc_config_install_path", &self.svc_config_install_path)?;
        map.serialize_entry("svc_data_path", &self.svc_data_path)?;
        map.serialize_entry("svc_files_path", &self.svc_files_path)?;
        map.serialize_entry("svc_static_path", &self.svc_static_path)?;
        map.serialize_entry("svc_var_path", &self.svc_var_path)?;
        map.serialize_entry("svc_pid_file", &self.svc_pid_file)?;
        map.serialize_entry("svc_run", &self.svc_run)?;
        map.serialize_entry("svc_user", &self.svc_user)?;
        map.serialize_entry("svc_group", &self.svc_group)?;

        map.end()
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        fs,
        io::{Read, Write},
        path::PathBuf,
    };

    use crate::hcore::package::PackageIdent;
    use serde_json;
    use tempfile::TempDir;

    use crate::templating::{
        config::{Cfg, PackageConfigPaths},
        test_helpers::*,
        TemplateRenderer,
    };

    ////////////////////////////////////////////////////////////////////////

    // These structs, functions, and impls are copied from
    // manager::service::config::test, and are used to create a
    // suitable `Cfg` struct for these tests.

    struct TestPkg {
        base_path: PathBuf,
    }

    impl TestPkg {
        fn new(tmp: &TempDir) -> Self {
            let pkg = Self {
                base_path: tmp.path().to_owned(),
            };

            fs::create_dir_all(pkg.default_config_dir())
                .expect("create deprecated user config dir");
            fs::create_dir_all(pkg.recommended_user_config_dir())
                .expect("create recommended user config dir");
            fs::create_dir_all(pkg.deprecated_user_config_dir())
                .expect("create default config dir");
            pkg
        }
    }

    impl PackageConfigPaths for TestPkg {
        fn name(&self) -> String {
            String::from("testing")
        }

        fn default_config_dir(&self) -> PathBuf {
            self.base_path.join("root")
        }

        fn recommended_user_config_dir(&self) -> PathBuf {
            self.base_path.join("user")
        }

        fn deprecated_user_config_dir(&self) -> PathBuf {
            self.base_path.join("svc")
        }
    }

    fn new_test_pkg() -> (TempDir, TestPkg) {
        let tmp = TempDir::new().expect("create temp dir");
        let pkg = TestPkg::new(&tmp);

        let default_toml = pkg.default_config_dir().join("default.toml");
        let mut buffer = fs::File::create(default_toml).expect("couldn't write file");
        buffer
            .write_all(
                br#"
foo = "bar"
baz = "boo"

[foobar]
one = 1
two = 2
"#,
            )
            .expect("Couldn't write default.toml");
        (tmp, pkg)
    }

    ////////////////////////////////////////////////////////////////////////

    /// Just create a basic RenderContext that could be used in tests.
    ///
    /// If you want to modify parts of it, it's easier to change
    /// things on a mutable reference.
    fn default_render_context<'a>() -> RenderContext<'a> {
        let ident = PackageIdent::new("core", "test_pkg", Some("1.0.0"), Some("20180321150416"));

        let deps = vec![
            PackageIdent::new("test", "pkg1", Some("1.0.0"), Some("20180321150416")),
            PackageIdent::new("test", "pkg2", Some("2.0.0"), Some("20180321150416")),
            PackageIdent::new("test", "pkg3", Some("3.0.0"), Some("20180321150416")),
        ];

        let mut env_hash = HashMap::new();
        env_hash.insert("PATH".into(), "/foo:/bar:/baz".into());
        env_hash.insert("SECRET".into(), "sooperseekrit".into());

        let mut export_hash = HashMap::new();
        export_hash.insert("blah".into(), "stuff.thing".into());
        export_hash.insert("port".into(), "test_port".into());

        let pkg = Package {
            ident: Cow::Owned(ident.clone()),
            // TODO (CM): have Pkg use FullyQualifiedPackageIdent, and
            // get origin, name, version, and release from it, rather
            // than storing each individually; I suspect that was just
            // for templating
            origin: Cow::Owned(ident.origin.clone()),
            name: Cow::Owned(ident.name.clone()),
            version: Cow::Owned(ident.version.clone().unwrap()),
            release: Cow::Owned(ident.release.clone().unwrap()),
            deps: Cow::Owned(deps),
            env: Cow::Owned(env_hash.into()),
            exposes: Cow::Owned(vec!["1234".into(), "8000".into(), "2112".into()]),
            exports: Cow::Owned(export_hash),
            path: Cow::Owned("my_path".into()),
            svc_path: Cow::Owned("svc_path".into()),
            svc_config_path: Cow::Owned("config_path".into()),
            svc_config_install_path: Cow::Owned("config_install_path".into()),
            svc_data_path: Cow::Owned("data_path".into()),
            svc_files_path: Cow::Owned("files_path".into()),
            svc_static_path: Cow::Owned("static_path".into()),
            svc_var_path: Cow::Owned("var_path".into()),
            svc_pid_file: Cow::Owned("pid_file".into()),
            svc_run: Cow::Owned("svc_run".into()),
            svc_user: Cow::Owned("hab".into()),
            svc_group: Cow::Owned("hab".into()),
        };

        // Not using _tmp_dir, but need it to prevent it from being
        // dropped before we make the Cfg
        let (_tmp_dir, test_pkg) = new_test_pkg();
        let cfg = Cfg::new(&test_pkg, None).expect("create config");

        RenderContext {
            pkg,
            cfg: Cow::Owned(cfg),
        }
    }

    /// Render the given template string using the given context,
    /// returning the result. This can help to verify that
    /// RenderContext data are accessible to users in the way we
    /// expect.
    fn render(template_content: &str, ctx: &RenderContext) -> String {
        let mut renderer = TemplateRenderer::new();
        renderer
            .register_template_string("testing", template_content)
            .expect("Could not register template content");
        renderer
            .render("testing", ctx)
            .expect("Could not render template")
    }

    ////////////////////////////////////////////////////////////////////////

    /// Reads a file containing real rendering context output from an
    /// actual Supervisor, prior to the refactoring to separate the
    /// serialization logic from the internal data structures.
    #[test]
    fn sample_context_is_valid() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("sample_render_context.json");

        let mut f = fs::File::open(path).expect("could not open sample_render_context.json");
        let mut json = String::new();
        f.read_to_string(&mut json)
            .expect("could not read sample_render_context.json");

        assert_valid(&json, "render_context_schema.json");
    }

    #[test]
    fn trivial_failure() {
        let state = validate_string(
            r#"{"svc":{},"pkg":{},"cfg":{},"svc":{},"bind":{}}"#,
            "render_context_schema.json",
        );
        assert!(
            !state.is_valid(),
            "Expected schema validation to fail, but it succeeded!"
        );
    }

    #[test]
    fn default_render_context_is_valid() {
        let render_context = default_render_context();
        let j = serde_json::to_string(&render_context).expect("can't serialize to JSON");
        assert_valid(&j, "render_context_schema.json");
    }

    #[test]
    fn renders_correctly() {
        let ctx = default_render_context();

        let output = render("{{pkg.origin}}", &ctx);

        assert_eq!(output, "core");
    }
}
