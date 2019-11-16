use crate::{error::Result,
            hcore::package::{metadata::DEFAULT_AGGREGATE_SEP,
                             PackageIdent,
                             PackageInstall}};
use std::{collections::{BTreeMap,
                        HashMap},
          env,
          path::Path};

use lazy_static::lazy_static;

pub fn start(idents: &[PackageIdent],
             fs_root_path: &Path,
             include_build_flags: bool,
             append_existing: bool)
             -> Result<()> {
    let env = if idents.len() == 1 {
        environment_for_ident(&idents[0], fs_root_path, include_build_flags)?
    } else {
        merged_environment_for_idents(idents, fs_root_path, include_build_flags)?
    };

    render_environment(env, append_existing);
    Ok(())
}

fn environment_for_ident(ident: &PackageIdent,
                         fs_root_path: &Path,
                         include_build_flags: bool)
                         -> Result<BTreeMap<String, String>> {
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    let mut env = pkg_install.environment_for_command()?;
    if include_build_flags {
        let mut build_env = pkg_install.build_environment()?;
        env.append(&mut build_env);
    }
    Ok(env)
}

// merged_environment_for_idents merges the runtime environments for
// the given idents.
//
// If the variable is a well-known aggregate value, we merge the
// values, removing duplicates and preserving the ordering.
//
// If the variable is not a well-known aggregate, the first entry
// wins.
fn merged_environment_for_idents(idents: &[PackageIdent],
                                 fs_root_path: &Path,
                                 include_build_flags: bool)
                                 -> Result<BTreeMap<String, String>> {
    // collect environments variables for each ident
    let mut pkg_envs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ident in idents.iter() {
        let pkg_env = environment_for_ident(ident, fs_root_path, include_build_flags)?;
        for (key, value) in pkg_env.iter() {
            match pkg_envs.get_mut(key) {
                Some(v) => {
                    v.push(value.to_string());
                }
                None => {
                    pkg_envs.insert(key.to_string(), vec![value.to_string()]);
                }
            };
        }
    }

    // build final enviroment map, deduplicating elements as we go.
    let mut env: BTreeMap<String, String> = BTreeMap::new();
    for (key, values) in pkg_envs.iter() {
        if is_known_aggregate(key) && values.len() > 1 {
            // NOTE(ssd) 2019-11-17: If this ends up being too
            // expensive we could drop the deduplication.
            let sep = &sep_for_envvar(&key);
            let mut markmap = HashMap::new();
            let value = values.iter()
                              .flat_map(|v| v.split(sep))
                              .filter(|v| markmap.insert(*v, ()).is_none())
                              .collect::<Vec<&str>>()
                              .join(sep);
            env.insert(key.to_string(), value);
        } else {
            env.insert(key.to_string(), values[0].clone());
        };
    }
    Ok(env)
}

lazy_static! {
    static ref WELL_KNOWN_AGGREGATES: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert("PATH", DEFAULT_AGGREGATE_SEP);
        m.insert("CFLAGS", ' ');
        m.insert("CPPFLAGS", ' ');
        m.insert("CXXFLAGS", ' ');
        m.insert("LDFLAGS", ' ');
        m.insert("LD_RUN_PATH", DEFAULT_AGGREGATE_SEP);
        m.insert("PKG_CONFIG_PATH", DEFAULT_AGGREGATE_SEP);

        #[cfg(windows)]
        {
            m.insert("INCLUDE", ';');
            m.insert("LIB", ';');
            m.insert("PATHEXT", ';');
            m.insert("PSModulePath", ';');
        }

        m.insert("CLASSPATH", ';');
        m.insert("PYTHONPATH", DEFAULT_AGGREGATE_SEP);
        m.insert("NODE_PATH", DEFAULT_AGGREGATE_SEP);
        m.insert("GOPATH", DEFAULT_AGGREGATE_SEP);
        m.insert("BUNDLE_PATH", DEFAULT_AGGREGATE_SEP);
        m.insert("BUNDLE_WITHOUT", DEFAULT_AGGREGATE_SEP);
        m.insert("GEM_PATH", DEFAULT_AGGREGATE_SEP);
        m.insert("RUBYLIB", DEFAULT_AGGREGATE_SEP);
        m.insert("RUBYPATH", DEFAULT_AGGREGATE_SEP);
        m.insert("PERL5LIB", DEFAULT_AGGREGATE_SEP);
        m.insert("CMAKE_FIND_ROOT_PATH", ';');
        m
    };
}

fn is_known_aggregate(varname: &str) -> bool { WELL_KNOWN_AGGREGATES.contains_key(varname) }

fn sep_for_envvar(varname: &str) -> String {
    WELL_KNOWN_AGGREGATES.get(varname)
                         .unwrap_or(&DEFAULT_AGGREGATE_SEP)
                         .to_string()
}

fn render_environment(environ: BTreeMap<String, String>, append_existing: bool) {
    for (key, mut value) in environ.into_iter() {
        if append_existing {
            if let Ok(existing_value) = env::var(&key) {
                value.push_str(&sep_for_envvar(&key));
                value.push_str(&existing_value);
            }
        }

        #[cfg(unix)]
        println!("export {}=\"{}\"", key, value);
        #[cfg(windows)]
        println!("$env:{}=\"{}\"", key, value);
    }
}
