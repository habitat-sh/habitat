/// Collect all the configuration data that is exposed to users, and render it.
use crate::{error::{Error,
                    Result},
            hcore::{self,
                    crypto,
                    fs::{self,
                         USER_CONFIG_FILE}},
            outputln,
            templating::{package::Pkg,
                         TemplateRenderer}};
use serde::{Serialize,
            Serializer};
use serde_json;
use serde_transcode;
use std::{self,
          borrow::Cow,
          env,
          fs::File,
          io::prelude::*,
          path::{Path,
                 PathBuf},
          result};
use toml;

static LOGKEY: &'static str = "CF";
static ENV_VAR_PREFIX: &'static str = "HAB";
/// The maximum TOML table merge depth allowed before failing the operation. The value here is
/// somewhat arbitrary (stack size cannot be easily computed beforehand and different libc
/// implementations will impose different size constraints), however a parallel data structure that
/// is deeper than this value crosses into overly complex territory when describing configuration
/// for a single service.
static TOML_MAX_MERGE_DEPTH: u16 = 30;
#[cfg(unix)]
pub const CONFIG_PERMISSIONS: u32 = 0o740;
#[cfg(unix)]
pub const CONFIG_DIR_PERMISSIONS: u32 = 0o770;

/// Describes the path to user configuration that is used by the
/// service.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserConfigPath {
    Recommended(PathBuf),
    Deprecated(PathBuf),
}

impl UserConfigPath {
    pub fn get_path(&self) -> &PathBuf {
        match self {
            UserConfigPath::Recommended(ref p) | UserConfigPath::Deprecated(ref p) => p,
        }
    }
}

impl From<UserConfigPath> for PathBuf {
    fn from(ucp: UserConfigPath) -> Self {
        match ucp {
            UserConfigPath::Recommended(p) | UserConfigPath::Deprecated(p) => p,
        }
    }
}

/// Trait for getting paths to directories where various configuration
/// files are expected to be.
pub trait PackageConfigPaths {
    /// Get name of the package (basically name part of package ident.
    fn name(&self) -> String;
    /// Get path to directory which holds default.toml.
    fn default_config_dir(&self) -> PathBuf;
    /// Get recommended path to directory which holds user.toml.
    fn recommended_user_config_dir(&self) -> PathBuf;
    /// Get deprecated path to directory which holds user.toml.
    fn deprecated_user_config_dir(&self) -> PathBuf;
}

impl PackageConfigPaths for Pkg {
    fn name(&self) -> String { self.name.clone() }

    fn default_config_dir(&self) -> PathBuf { self.path.clone() }

    fn recommended_user_config_dir(&self) -> PathBuf { fs::user_config_path(&self.name) }

    fn deprecated_user_config_dir(&self) -> PathBuf { self.svc_path.clone() }
}

#[derive(Clone, Debug)]
pub struct Cfg {
    /// Default level configuration loaded by a Package's `default.toml`
    pub default: Option<toml::value::Table>,
    /// User level configuration loaded by a Service's `user.toml`
    pub user: Option<toml::value::Table>,
    /// Gossip level configuration loaded by a census group
    pub gossip: Option<toml::value::Table>,
    /// Environment level configuration loaded by the Supervisor's process environment
    pub environment: Option<toml::value::Table>,
    /// Source of the user configuration
    pub user_config_path: UserConfigPath,
    /// Last known incarnation number of the census group's service config
    pub gossip_incarnation: u64,
    /// The path to an optional dev-time configuration directory that
    /// is being used.
    override_config_dir: Option<PathBuf>,
}

impl Cfg {
    pub fn new<P>(package: &P, config_from: Option<&PathBuf>) -> Result<Cfg>
        where P: PackageConfigPaths
    {
        let override_config_dir = config_from.and_then(|c| Some(c.clone()));
        let default = {
            let pkg_root = match override_config_dir {
                Some(ref path) => Cow::Borrowed(path),
                None => Cow::Owned(package.default_config_dir()),
            };
            Self::load_default(pkg_root.as_ref())?
        };
        let user_config_path = Self::determine_user_config_path(package);
        let user = Self::load_user(user_config_path.get_path())?;
        let environment = Self::load_environment(&package.name())?;
        Ok(Self { default,
                  user,
                  gossip: None,
                  environment,
                  gossip_incarnation: 0,
                  user_config_path,
                  override_config_dir })
    }

    /// Validates a service configuration against a configuration interface.
    ///
    /// Returns `None` if valid and `Some` containing a list of errors if invalid.
    pub fn validate(interface: &toml::value::Table,
                    cfg: &toml::value::Table)
                    -> Option<Vec<String>> {
        let mut errors = vec![];
        for key in cfg.keys() {
            if !interface.contains_key(key) {
                errors.push(format!("Unknown key: {}", key));
            }
        }
        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    /// A structured interface which describes configuration keys which are configurable and their
    /// optional default values.
    pub fn interface(&self) -> Option<&toml::value::Table> {
        // JW TODO: For now let's use `default.toml` as it is for the interface. In the future,
        // we will need to be able to describe more than just the key value relationship that
        // `default.toml` provides. We will need to be able to describe things like:
        //
        // * Keys which have no default
        // * Key with a default value
        // * Keys which only accept certain values
        // * Allowed types for a key
        self.default.as_ref()
    }

    /// Updates the default layer of the configuration when a service
    /// is updated (because the new release may have changed the
    /// contents and / or structure of the configuration).
    ///
    /// Returns `Ok(true)` if the default layer was actually changed,
    /// which can be used as a signal to rebuild templated contents in
    /// hooks and configuration files.
    ///
    /// Note that if you're using `config_from`, then changes in the
    /// incoming packages won't be reflected.
    pub fn update_defaults_from_package<P>(&mut self, package: &P) -> Result<bool>
        where P: PackageConfigPaths
    {
        let incoming_defaults = {
            let pkg_root = match self.override_config_dir {
                Some(ref path) => Cow::Borrowed(path),
                None => Cow::Owned(package.default_config_dir()),
            };
            Self::load_default(pkg_root.as_ref())?
        };

        if incoming_defaults != self.default {
            self.default = incoming_defaults;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Updates the service configuration with data from a census group
    pub fn set_gossip(&mut self, incarnation: u64, gossip: toml::value::Table) {
        self.gossip_incarnation = incarnation;
        self.gossip = Some(gossip);
    }

    /// Returns a subset of the overall configuration which intersects with the given package
    /// exports.
    pub fn to_exported(&self, pkg: &Pkg) -> Result<toml::value::Table> {
        let mut map = toml::value::Table::default();
        let cfg = toml::Value::try_from(&self).expect("Cfg -> TOML conversion");;
        for (key, path) in pkg.exports.iter() {
            let mut curr = &cfg;
            let mut found = false;

            // JW TODO: the TOML library only provides us with a
            // function to retrieve a value with a path which returns a
            // reference. We actually want the value for ourselves.
            // Let's improve this later to avoid allocating twice.
            for field in path.split('.') {
                match curr.get(field) {
                    Some(val) => {
                        curr = val;
                        found = true;
                    }
                    None => found = false,
                }
            }

            if found {
                map.insert(key.clone(), curr.clone());
            }
        }
        Ok(map)
    }

    fn load_toml_file<T1, T2>(dir: T1, file: T2) -> Result<Option<toml::value::Table>>
        where T1: AsRef<Path>,
              T2: AsRef<Path>
    {
        let filename = file.as_ref();
        let path = dir.as_ref().join(&filename);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                debug!("Failed to open '{}', {}, {}",
                       filename.display(),
                       path.display(),
                       e);
                return Ok(None);
            }
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {
                let toml = toml::de::from_str(&config).map_err(Error::TomlParser)?;
                Ok(Some(toml))
            }
            Err(e) => {
                outputln!("Failed to read '{}', {}, {}",
                          filename.display(),
                          path.display(),
                          e);
                Ok(None)
            }
        }
    }

    fn load_default<T>(config_from: T) -> Result<Option<toml::value::Table>>
        where T: AsRef<Path>
    {
        Self::load_toml_file(config_from, "default.toml")
    }

    fn determine_user_config_path<P: PackageConfigPaths>(package: &P) -> UserConfigPath {
        let recommended_dir = package.recommended_user_config_dir();
        let recommended_path = recommended_dir.join(USER_CONFIG_FILE);
        if recommended_path.exists() {
            return UserConfigPath::Recommended(recommended_dir);
        }
        debug!("'{}' at {} does not exist",
               USER_CONFIG_FILE,
               recommended_path.display());
        let deprecated_dir = package.deprecated_user_config_dir();
        let deprecated_path = deprecated_dir.join(USER_CONFIG_FILE);
        if deprecated_path.exists() {
            outputln!("The user configuration location at {} is deprecated, consider putting it \
                       in {}",
                      deprecated_path.display(),
                      recommended_path.display(),);
            return UserConfigPath::Deprecated(deprecated_dir);
        }
        debug!("'{}' at {} does not exist",
               USER_CONFIG_FILE,
               deprecated_path.display());
        UserConfigPath::Recommended(recommended_dir)
    }

    fn load_user<T>(path: T) -> Result<Option<toml::value::Table>>
        where T: AsRef<Path>
    {
        Self::load_toml_file(path, USER_CONFIG_FILE)
    }

    /// Reloads the user configuration file.
    pub fn reload_user(&mut self) -> Result<()> {
        let user = Self::load_user(self.user_config_path.get_path())?;
        self.user = user;
        Ok(())
    }

    fn load_environment(package_name: &str) -> Result<Option<toml::value::Table>> {
        let var_name = format!("{}_{}", ENV_VAR_PREFIX, package_name).to_ascii_uppercase()
                                                                     .replace("-", "_");
        match env::var(&var_name) {
            Ok(config) => {
                // If we've got an environment variable, we'll parsing
                // as TOML first, since that's easiest.
                match toml::de::from_str(&config) {
                    Ok(toml) => {
                        return Ok(Some(toml));
                    }
                    Err(err) => debug!("Attempted to parse env config as toml and failed {}", err),
                }

                // We know we're not dealing with TOML, so we'll
                // assume it's JSON. Since we're currently decoding to
                // toml::value::Table, and there isn't really an easy
                // way to directly go from a JSON string to that, we
                // first transcode the JSON string into a TOML string,
                // and then deserialize from THAT.
                //
                // Not the greatest, but it works.
                let (as_toml, transcode_result) = {
                    let mut buffer = String::new();
                    let mut deserializer = serde_json::Deserializer::from_str(&config);
                    let res = {
                        let mut serializer = toml::Serializer::new(&mut buffer);
                        serde_transcode::transcode(&mut deserializer, &mut serializer)
                    };
                    (buffer, res)
                };
                match transcode_result {
                    Ok(()) => {
                        // it's TOML now, so turn it into a TOML table
                        match toml::de::from_str(&as_toml) {
                            Ok(toml) => {
                                return Ok(Some(toml));
                            }
                            Err(err) => {
                                // Note: it should be impossible to
                                // get down here
                                debug!("Attempted to reparse env config as toml and failed {}", err)
                            }
                        }
                    }
                    Err(err) => debug!("Attempted to parse env config as json and failed {}", err),
                }

                // It's neither TOML nor JSON, so bail out
                Err(Error::BadEnvConfig(var_name))
            }
            Err(e) => {
                debug!("Looking up environment variable {} failed: {:?}",
                       var_name, e);
                Ok(None)
            }
        }
    }
}

impl Serialize for Cfg {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut table = toml::value::Table::new();
        if let Some(ref default_cfg) = self.default {
            if let Err(err) = toml_merge(&mut table, default_cfg) {
                outputln!("Error merging default-cfg into config, {}", err);
            }
        }
        if let Some(ref env_cfg) = self.environment {
            if let Err(err) = toml_merge(&mut table, env_cfg) {
                outputln!("Error merging environment-cfg into config, {}", err);
            }
        }
        if let Some(ref user_cfg) = self.user {
            if let Err(err) = toml_merge(&mut table, user_cfg) {
                outputln!("Error merging user-cfg into config, {}", err);
            }
        }
        if let Some(ref gossip_cfg) = self.gossip {
            if let Err(err) = toml_merge(&mut table, gossip_cfg) {
                outputln!("Error merging gossip-cfg into config, {}", err);
            }
        }

        toml::ser::tables_last(&table, serializer)
    }
}

#[derive(Debug)]
/// Renders configuration templates into config files.
pub struct CfgRenderer(TemplateRenderer);

impl CfgRenderer {
    /// Create a new `CfgRenderer` and load template files from a
    /// configuration directory, if it exists.
    pub fn new<T>(templates_path: T) -> Result<Self>
        where T: AsRef<Path>
    {
        if templates_path.as_ref().is_dir() {
            load_templates(templates_path.as_ref(),
                           &PathBuf::new(),
                           TemplateRenderer::new()).map(CfgRenderer)
        } else {
            Ok(CfgRenderer(TemplateRenderer::new()))
        }
    }

    /// Compile and write all configuration files to the configuration directory.
    ///
    /// Returns `true` if the configuration has changed.
    pub fn compile<P, T>(&self,
                         service_group_name: &str,
                         pkg: &Pkg,
                         render_path: P,
                         ctx: &T)
                         -> Result<bool>
        where P: AsRef<Path>,
              T: Serialize
    {
        // JW TODO: This function is loaded with IO errors that will be converted a Supervisor
        // error resulting in the end-user not knowing what the fuck happned at all. We need to go
        // through this and pipe the service group through to let people know which service is
        // having issues and be more descriptive about what happened.

        let mut changed = false;
        for template in self.0.get_templates().keys() {
            let compiled = self.0.render(&template, ctx)?;
            let compiled_hash = crypto::hash::hash_string(&compiled);
            let cfg_dest = render_path.as_ref().join(&template);
            let file_hash = match crypto::hash::hash_file(&cfg_dest) {
                Ok(file_hash) => file_hash,
                Err(e) => {
                    debug!("Cannot read the file in order to hash it: {}", e);
                    String::new()
                }
            };
            changed |= if file_hash.is_empty() {
                debug!("Configuration {} does not exist; restarting",
                       cfg_dest.display());

                ensure_directory_structure(render_path.as_ref(),
                                           &cfg_dest,
                                           &pkg.svc_user,
                                           &pkg.svc_group)?;
                write_templated_file(&cfg_dest, &compiled, &pkg.svc_user, &pkg.svc_group)?;
                outputln!(
                    preamble service_group_name,
                    "Created configuration file {}",
                    cfg_dest.display()
                );

                true
            } else if file_hash == compiled_hash {
                debug!("Configuration {} {} has not changed; not restarting.",
                       cfg_dest.display(),
                       file_hash);
                false
            } else {
                debug!("Configuration {} has changed; restarting",
                       cfg_dest.display());
                write_templated_file(&cfg_dest, &compiled, &pkg.svc_user, &pkg.svc_group)?;
                outputln!(
                    preamble service_group_name,
                    "Modified configuration file {}",
                    cfg_dest.display()
                );
                true
            };
        }
        Ok(changed)
    }
}

// Recursively merges the `other` TOML table into `me`
fn toml_merge(me: &mut toml::value::Table, other: &toml::value::Table) -> Result<()> {
    toml_merge_recurse(me, other, 0)
}

fn toml_merge_recurse(me: &mut toml::value::Table,
                      other: &toml::value::Table,
                      depth: u16)
                      -> Result<()> {
    if depth > TOML_MAX_MERGE_DEPTH {
        return Err(Error::TomlMergeError(format!("Max recursive merge depth \
                                                  of {} exceeded.",
                                                 TOML_MAX_MERGE_DEPTH)));
    }

    for (key, other_value) in other.iter() {
        if is_toml_value_a_table(key, me) && is_toml_value_a_table(key, other) {
            let mut me_at_key = match *(me.get_mut(key).expect("Key should exist in Table")) {
                toml::Value::Table(ref mut t) => t,
                _ => {
                    return Err(Error::TomlMergeError(format!("Value at key {} should be \
                                                              a Table",
                                                             &key)));
                }
            };
            toml_merge_recurse(&mut me_at_key,
                               other_value.as_table()
                                          .expect("TOML Value should be a Table"),
                               depth + 1)?;
        } else {
            me.insert(key.clone(), other_value.clone());
        }
    }
    Ok(())
}

fn is_toml_value_a_table(key: &str, table: &toml::value::Table) -> bool {
    match table.get(key) {
        None => false,
        Some(value) => value.as_table().is_some(),
    }
}

#[cfg(unix)]
fn set_permissions(path: &Path, user: &str, group: &str) -> hcore::error::Result<()> {
    use crate::hcore::{os::users,
                       util::posix_perm};

    if users::can_run_services_as_svc_user() {
        posix_perm::set_owner(path, &user, &group)?;
    }

    let permissions = if path.is_dir() {
        CONFIG_DIR_PERMISSIONS
    } else {
        CONFIG_PERMISSIONS
    };
    posix_perm::set_permissions(&path, permissions)
}

#[cfg(windows)]
fn set_permissions(path: &Path, _user: &str, _group: &str) -> hcore::error::Result<()> {
    use crate::hcore::util::win_perm;

    win_perm::harden_path(path)
}

/// Recursively walk the configuration directory and subdirectories to
/// construct the list of template files
///
/// `dir` should be a directory that exists.
fn load_templates(dir: &Path,
                  context: &Path,
                  mut template: TemplateRenderer)
                  -> Result<TemplateRenderer> {
    for entry in std::fs::read_dir(dir)?.filter_map(|entry| entry.ok()) {
        // We're storing the pathname relative to the input config directory
        // as the identifier for the template
        let relative_path = context.join(&entry.file_name());
        match entry.file_type() {
            Ok(file_type) if file_type.is_file() => {
                // JW TODO: This error needs improvement. TemplateFileError is too generic.
                template.register_template_file(&relative_path.to_string_lossy(), &entry.path())
                        .map_err(|e| Error::TemplateFileError(Box::new(e)))?;
            }
            Ok(file_type) if file_type.is_dir() => {
                template = load_templates(&entry.path(), &relative_path, template)?
            }
            Ok(file_type) => trace!("Skipping non file/directory entry: {:?}", file_type),
            Err(e) => debug!("Failed to get file metadata for {:?} : {}", entry, e),
        }
    }
    Ok(template)
}

/// Create the appropriate directories between a `root` directory
/// and a file within that directory structure that we're about
/// to create.
///
/// - `root` must be a directory
/// - `file` must be contained within the `root` directory at an arbitrary depth
fn ensure_directory_structure(root: &Path, file: &Path, user: &str, group: &str) -> Result<()> {
    // We check that `file` is below `root` in the directory structure and
    // that `root` exists, so that we don't create arbitrary directory
    // structures on disk
    assert!(root.is_dir());
    assert!(file.starts_with(&root));

    let dir = file.parent().unwrap();

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
        for anc in dir.ancestors().take_while(|&d| d != root) {
            set_permissions(&anc, &user, &group)?;
        }
    }
    Ok(())
}

fn write_templated_file(path: &Path, compiled: &str, user: &str, group: &str) -> Result<()> {
    File::create(path).and_then(|mut file| file.write_all(compiled.as_bytes()))?;

    set_permissions(&path, &user, &group)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{error::Error,
                hcore::{os::users,
                        package::{PackageIdent,
                                  PackageInstall}},
                templating::{context::RenderContext,
                             test_helpers::*}};
    use std::{env,
              fs::{self,
                   OpenOptions}};
    use tempfile::TempDir;
    use toml;

    fn curr_username() -> String {
        users::get_current_username().expect("Can get current username")
    }

    fn curr_groupname() -> String {
        users::get_current_groupname().expect("Can get current groupname")
    }

    fn toml_from_str(content: &str) -> toml::value::Table {
        toml::from_str(content).unwrap_or_else(|_| {
                                   panic!("Content should parse as TOML: {}", content)
                               })
    }

    #[test]
    fn merge_with_empty_me_table() {
        let mut me = toml_from_str("");
        let other = toml_from_str(
                                  r#"
            fruit = "apple"
            veggie = "carrot"
            "#,
        );
        let expected = other.clone();
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_empty_other_table() {
        let mut me = toml_from_str(
                                   r#"
            fruit = "apple"
            veggie = "carrot"
            "#,
        );
        let other = toml_from_str("");
        let expected = me.clone();
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_shallow_tables() {
        let mut me = toml_from_str(
                                   r#"
            fruit = "apple"
            veggie = "carrot"
            awesomeness = 10
            "#,
        );
        let other = toml_from_str(
                                  r#"
            fruit = "orange"
            awesomeness = 99
            "#,
        );
        let expected = toml_from_str(
                                     r#"
            fruit = "orange"
            veggie = "carrot"
            awesomeness = 99
            "#,
        );
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_differing_value_types() {
        let mut me = toml_from_str(
                                   r#"
            fruit = "apple"
            veggie = "carrot"
            awesome_things = ["carrots", "kitties", "unicorns"]
            heat = 42
            "#,
        );
        let other = toml_from_str(
                                  r#"
            heat = "hothothot"
            awesome_things = "habitat"
            "#,
        );
        let expected = toml_from_str(
                                     r#"
            heat = "hothothot"
            fruit = "apple"
            veggie = "carrot"
            awesome_things = "habitat"
            "#,
        );
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_table_values() {
        let mut me = toml_from_str(
                                   r#"
            frubnub = "foobar"

            [server]
            some-details = "initial"
            port = 1000
            "#,
        );
        let other = toml_from_str(
                                  r#"
            [server]
            port = 5000
            more-details = "yep"
            "#,
        );
        let expected = toml_from_str(
                                     r#"
            frubnub = "foobar"

            [server]
            port = 5000
            some-details = "initial"
            more-details = "yep"
            "#,
        );
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_deep_table_values() {
        let mut me = toml_from_str(
                                   r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "carrot"
            [a.b.c.d.e.f.foxtrot]
            fancy = "fork"
            "#,
        );
        let other = toml_from_str(
                                  r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "beef"
            [a.b.c.d.e.f.foxtrot]
            fancy = "feast"
            funny = "farm"
            "#,
        );
        let expected = toml_from_str(
                                     r#"
            [a.b.c.d.e.f.foxtrot]
            funny = "farm"
            fancy = "feast"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad]
            stew = "beef"
            "#,
        );
        toml_merge(&mut me, &other).unwrap();

        assert_eq!(me, expected);
    }

    #[test]
    fn merge_with_dangerously_deep_table_values() {
        let mut me = toml_from_str(
                                   r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad.ae.af]
            stew = "carrot"
            "#,
        );
        let other = toml_from_str(
                                  r#"
            [a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z.aa.ab.ac.ad.ae.af]
            stew = "beef"
            "#,
        );

        match toml_merge(&mut me, &other) {
            Err(e) => {
                match e {
                    Error::TomlMergeError(_) => assert!(true),
                    _ => panic!("Should fail with Error::TomlMergeError"),
                }
            }
            Ok(_) => panic!("Should not complete successfully"),
        }
    }

    struct TestPkg {
        base_path: PathBuf,
    }

    impl TestPkg {
        fn new(tmp: &TempDir) -> Self {
            let pkg = Self { base_path: tmp.path().to_owned(), };

            fs::create_dir_all(pkg.default_config_dir()).expect("create deprecated user config \
                                                                 dir");
            fs::create_dir_all(pkg.recommended_user_config_dir()).expect("create recommended \
                                                                          user config dir");
            fs::create_dir_all(pkg.deprecated_user_config_dir()).expect("create default config \
                                                                         dir");
            pkg
        }
    }

    impl PackageConfigPaths for TestPkg {
        fn name(&self) -> String { String::from("testing") }

        fn default_config_dir(&self) -> PathBuf { self.base_path.join("root") }

        fn recommended_user_config_dir(&self) -> PathBuf { self.base_path.join("user") }

        fn deprecated_user_config_dir(&self) -> PathBuf { self.base_path.join("svc") }
    }

    struct CfgTestData {
        // We hold tmp here only to make sure that the temporary
        // directory gets deleted at the end of the test.
        #[allow(dead_code)]
        tmp: TempDir,
        pkg: TestPkg,
        rucp: PathBuf,
        ducp: PathBuf,
    }

    impl CfgTestData {
        fn new() -> Self {
            let tmp = TempDir::new().expect("create temp dir");
            let pkg = TestPkg::new(&tmp);
            let rucp = pkg.recommended_user_config_dir().join(USER_CONFIG_FILE);
            let ducp = pkg.deprecated_user_config_dir().join(USER_CONFIG_FILE);
            Self { tmp,
                   pkg,
                   rucp,
                   ducp }
        }
    }

    fn write_toml<P: AsRef<Path>>(path: &P, text: &str) {
        let mut file = OpenOptions::new().write(true)
                                         .create(true)
                                         .truncate(true)
                                         .open(path)
                                         .expect("create toml file");
        file.write_all(text.as_bytes())
            .expect("write raw toml value");
        file.flush().expect("flush changes in toml file");
    }

    #[test]
    fn default_to_recommended_user_toml_if_missing() {
        let cfg_data = CfgTestData::new();
        let cfg = Cfg::new(&cfg_data.pkg, None).expect("create config");

        assert_eq!(cfg.user_config_path,
                   UserConfigPath::Recommended(cfg_data.pkg.recommended_user_config_dir()));
        assert!(cfg.user.is_none());
    }

    #[test]
    fn load_deprecated_user_toml() {
        let cfg_data = CfgTestData::new();
        let toml = "foo = 42";
        write_toml(&cfg_data.ducp, toml);
        let cfg = Cfg::new(&cfg_data.pkg, None).expect("create config");

        assert_eq!(cfg.user_config_path,
                   UserConfigPath::Deprecated(cfg_data.pkg.deprecated_user_config_dir()));
        assert_eq!(cfg.user, Some(toml_from_str(toml)));
    }

    #[test]
    fn load_recommended_user_toml() {
        let cfg_data = CfgTestData::new();
        let toml = "foo = 42";
        write_toml(&cfg_data.rucp, toml);
        let cfg = Cfg::new(&cfg_data.pkg, None).expect("create config");

        assert_eq!(cfg.user_config_path,
                   UserConfigPath::Recommended(cfg_data.pkg.recommended_user_config_dir()));
        assert_eq!(cfg.user, Some(toml_from_str(toml)));
    }

    #[test]
    fn prefer_recommended_to_deprecated() {
        let cfg_data = CfgTestData::new();
        let toml = "foo = 42";
        write_toml(&cfg_data.rucp, toml);
        write_toml(&cfg_data.ducp, "foo = 13");
        let cfg = Cfg::new(&cfg_data.pkg, None).expect("create config");

        assert_eq!(cfg.user_config_path,
                   UserConfigPath::Recommended(cfg_data.pkg.recommended_user_config_dir()));
        assert_eq!(cfg.user, Some(toml_from_str(toml)));
    }

    #[test]
    fn keep_loading_deprecated_after_initial_load() {
        let cfg_data = CfgTestData::new();
        let mut toml = "foo = 13";
        write_toml(&cfg_data.ducp, toml);
        let mut cfg = Cfg::new(&cfg_data.pkg, None).expect("create config");

        assert_eq!(cfg.user_config_path,
                   UserConfigPath::Deprecated(cfg_data.pkg.deprecated_user_config_dir()));
        assert_eq!(cfg.user, Some(toml_from_str(toml)));

        toml = "foo = 85";
        write_toml(&cfg_data.ducp, toml);
        write_toml(&cfg_data.rucp, "foo = 42");
        cfg.reload_user().expect("reload user config");

        assert_eq!(cfg.user_config_path,
                   UserConfigPath::Deprecated(cfg_data.pkg.deprecated_user_config_dir()));
        assert_eq!(cfg.user, Some(toml_from_str(toml)));
    }

    #[test]
    fn serialize_config() {
        let concrete_path = TempDir::new().expect("create temp dir");
        let pkg = TestPkg::new(&concrete_path);
        let mut cfg = Cfg::new(&pkg, None).expect("Could not create config");
        let default_toml = "shards = []\n\n[datastore]\ndatabase = \
                            \"builder_originsrv\"\npassword = \"\"\nuser = \"hab\"\n";

        cfg.default = Some(toml::from_str(default_toml).unwrap());
        assert_eq!(default_toml, toml::to_string(&cfg).unwrap());
    }

    // env_key: the name of the environment variable the config should
    //     be read from
    // package_name: the name of the package that would read
    //     environment configuration from `env_key`.
    // input_config: the value of the environment variable `env_key`;
    //     can be either JSON or TOML
    // expected_config_as_toml: for validation purposes; this should
    //     be the TOML version of `input_config`, since we always read to
    //     TOML, regardless of the input format.
    fn test_expected_successful_environment_parsing(env_key: &str,
                                                    package_name: &str,
                                                    input_config: &str,
                                                    expected_config_as_toml: &str) {
        env::set_var(env_key, &input_config);

        let expected = toml_from_str(expected_config_as_toml);
        let result = Cfg::load_environment(&package_name.to_string());

        // Clean up the environment after ourselves
        env::remove_var(env_key);

        match result {
            Ok(Some(actual)) => {
                assert_eq!(actual, expected);
            }
            _ => {
                panic!("Expected '{:?}', but got {:?}", expected, result);
            }
        }
    }

    #[test]
    fn can_parse_toml_environment_config() {
        test_expected_successful_environment_parsing("HAB_TESTING_TOML",
                                                     "testing-toml",
                                                     "port = 1234",
                                                     "port = 1234");
    }

    #[test]
    fn can_parse_json_environment_config() {
        test_expected_successful_environment_parsing("HAB_TESTING_JSON",
                                                     "testing-json",
                                                     "{\"port\": 1234}",
                                                     "port = 1234");
    }

    #[test]
    fn parse_environment_config_that_is_neither_toml_nor_json_fails() {
        let key = "HAB_TESTING_TRASH";
        let config = "{\"port: 1234 what even is this!!!!?! =";

        env::set_var(key, &config);

        let result = Cfg::load_environment(&"testing-trash".to_string());

        // Clean up the environment after ourselves
        env::remove_var(key);

        assert!(result.is_err(), "Expected an error: got {:?}", result);
    }

    #[test]
    fn no_environment_config_is_fine() {
        match Cfg::load_environment(
            &"omg-there-wont-be-an-environment-variable-for-this".to_string(),
        ) {
            Ok(None) => (),
            other => {
                panic!("Expected Ok(None); got {:?}", other);
            }
        }
    }

    #[test]
    fn write_template_file_simple() {
        let tmp = TempDir::new().expect("create temp dir");
        let template_dir = tmp.path().join("output");
        fs::create_dir_all(&template_dir).expect("create output dir");

        let file = template_dir.join("config.cfg");
        let contents = "foo\nbar\n";

        assert_eq!(file.exists(), false);
        write_templated_file(&file, &contents, &curr_username(), &curr_groupname())
            .expect("writes file");
        assert!(file.exists());
    }

    #[test]
    fn write_template_file_directory() {
        let tmp = TempDir::new().expect("create temp dir");
        let template_dir = tmp.path().join("output");
        fs::create_dir_all(&template_dir).expect("create output dir");

        let file = template_dir.join("foo/config.cfg");
        let contents = "foo\nbar\n";

        assert_eq!(file.exists(), false);

        ensure_directory_structure(&template_dir, &file, &curr_username(), &curr_groupname())
            .expect("create output dir structure");
        write_templated_file(&file, &contents, &curr_username(), &curr_groupname())
            .expect("writes file");
        assert!(file.exists());
        assert_eq!(file_content(file), contents);
    }

    #[test]
    #[cfg(unix)]
    fn write_template_file_no_perms() {
        use crate::hcore::util::posix_perm;
        const NO_PERMISSIONS: u32 = 0o000;

        let tmp = TempDir::new().expect("create temp dir");
        let template_dir = tmp.path().join("output");
        fs::create_dir_all(&template_dir).expect("create output dir");
        posix_perm::set_permissions(&template_dir, NO_PERMISSIONS).unwrap();

        let file = template_dir.join("config.cfg");
        let contents = "foo\nbar\n";

        assert_eq!(file.exists(), false);
        assert!(
            write_templated_file(&file, &contents, &curr_username(), &curr_groupname()).is_err()
        );
    }

    #[test]
    /// Check we can load templates out of a set of hierarchical directories
    /// and that the template keys correspond to the relative file names from
    /// the top-level config dir
    fn test_load_templates_recursive() {
        let tmp = TempDir::new().expect("create temp dir");
        let input_dir = tmp.path().join("input");

        let dir_a = input_dir.join("dir_a");
        let dir_b = input_dir.join("dir_b");
        let dir_c = dir_b.join("dir_c");
        fs::create_dir_all(&dir_a).expect("create dir_a");
        fs::create_dir_all(&dir_c).expect("create dir_b and dir_c");

        create_with_content(&dir_a.join("foo.txt"), "Hello world!");
        create_with_content(&dir_b.join("bar.txt"), "Hello world!");
        create_with_content(&dir_c.join("baz.txt"), "Hello world!");

        let renderer = load_templates(&input_dir, &PathBuf::new(), TemplateRenderer::new())
            .expect("visit config dirs");

        let expected_keys = vec![PathBuf::from("dir_a").join("foo.txt"),
                                 PathBuf::from("dir_b").join("bar.txt"),
                                 PathBuf::from("dir_b").join("dir_c").join("baz.txt"),];
        let templates = renderer.get_templates();
        assert_eq!(templates.len(), 3);

        for key in expected_keys {
            let str_key = key.to_string_lossy().into_owned();
            assert!(templates.contains_key(&str_key));
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    #[should_panic(expected = "Not a directory")]
    #[cfg(target_os = "windows")]
    #[should_panic(expected = "The directory name is invalid")]
    /// Check we get an error if we pass in a file to `load_templates`
    fn test_load_templates_file() {
        let tmp = TempDir::new().expect("create temp dir");

        let file = tmp.path().join("bar.txt");
        create_with_content(&file, "Hello world!");

        load_templates(&file, &PathBuf::new(), TemplateRenderer::new())?;
    }

    #[test]
    fn test_compile_recursive_config_dir() {
        let root = TempDir::new().expect("create temp dir").into_path();

        // Setup a dummy package directory with a config file inside
        // a directory structure
        let pkg_dir = root.join("pkg/testing/test");
        fs::create_dir_all(&pkg_dir).expect("create pkg dir");
        let pg_id = PackageIdent::new("testing", "test", Some("1.0.0"), Some("20170712000000"));
        let pkg_install = PackageInstall::new_from_parts(pg_id.clone(),
                                                         pkg_dir.clone(),
                                                         pkg_dir.clone(),
                                                         pkg_dir.clone());
        let toml_path = pkg_dir.join("default.toml");
        create_with_content(toml_path, "message = \"Hello\"");

        let config_dir = pkg_dir.join("config");
        let deep_config_dir = config_dir.join("dir_a").join("dir_b");
        fs::create_dir_all(&deep_config_dir).expect("create config/dir_a/dir_b");
        create_with_content(deep_config_dir.join("config.txt"),
                            "config message is {{cfg.message}}");

        // Setup context for loading and compiling templates
        let output_dir = root.join("output");
        fs::create_dir_all(&output_dir).expect("create output dir");

        let pkg = Pkg::from_install(&pkg_install).unwrap();
        let cfg = Cfg::new(&pkg, None).unwrap();
        let ctx = RenderContext::new(&pkg, &cfg);

        // Load templates from pkg config dir, and compile then into
        // the output directory
        let renderer = CfgRenderer::new(&config_dir).expect("create cfg renderer");
        renderer.compile("test", &pkg, &output_dir, &ctx)
                .expect("compile");
        let deep_output_dir = output_dir.join("dir_a").join("dir_b");

        assert_eq!(file_content(deep_output_dir.join("config.txt")),
                   "config message is Hello");
    }
}
