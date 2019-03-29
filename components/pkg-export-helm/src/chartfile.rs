use clap;
use failure::SyncFailure;
use handlebars::Handlebars;
use serde_json;

use crate::{export_docker::Result,
            hcore::package::PackageIdent};

use crate::maintainer::Maintainer;

pub const DEFAULT_VERSION: &str = "0.0.1";

// Helm chart file template
const CHARTFILE: &str = include_str!("../defaults/HelmChartFile.hbs");

pub struct ChartFile {
    pub name:        String,
    pub version:     String,
    pub description: Option<String>,
    pub app_version: Option<String>,
    pub home:        Option<String>,
    pub icon:        Option<String>,
    pub deprecated:  bool,
    pub keywords:    Vec<String>,
    pub sources:     Vec<String>,
    pub maintainers: Vec<Maintainer>,
}

impl ChartFile {
    pub fn new_from_cli_matches(matches: &clap::ArgMatches<'_>,
                                pkg_ident: &PackageIdent)
                                -> Result<Self> {
        let name = matches.value_of("CHART")
                          .unwrap_or(&pkg_ident.name)
                          .to_string();
        let pkg_version = pkg_ident.version.as_ref();
        let version = matches.value_of("VERSION")
                             .or_else(|| pkg_version.map(|s| s.as_ref()))
                             .unwrap_or(DEFAULT_VERSION)
                             .to_owned();
        let app_version = pkg_version.map(|v| {
                                         pkg_ident.release
                                                  .as_ref()
                                                  .map(|r| format!("{}-{}", v, r))
                                                  .unwrap_or_else(|| v.to_string())
                                     });
        let description = matches.value_of("DESCRIPTION").map(|s| s.to_owned());
        let home = matches.value_of("HOME").map(|s| s.to_owned());
        let icon = matches.value_of("ICON").map(|s| s.to_owned());
        let deprecated = matches.is_present("DEPRECATED");
        let keywords = matches.values_of("KEYWORD")
                              .map(|args| args.map(|k| k.to_owned()).collect())
                              .unwrap_or_default();
        let sources = matches.values_of("SOURCE")
                             .map(|args| args.map(|k| k.to_owned()).collect())
                             .unwrap_or_default();
        let maintainers = Maintainer::from_args(&matches)?;

        Ok(ChartFile { name,
                       version,
                       description,
                       app_version,
                       home,
                       icon,
                       deprecated,
                       keywords,
                       sources,
                       maintainers })
    }

    // TODO: Implement TryInto trait instead when it's in stable std crate
    pub fn to_string(&self) -> Result<String> {
        let mut maintainers: Vec<serde_json::Value> = vec![];
        for maintainer in &self.maintainers {
            maintainers.push(maintainer.to_json());
        }

        let json = json!({
            "name": self.name,
            "version": self.version,
            "description": self.description,
            "appVersion": self.app_version,
            "home": self.home,
            "icon": self.icon,
            "deprecated": self.deprecated,
            "keywords": self.keywords,
            "sources": self.sources,
            "maintainers": maintainers,
        });

        Handlebars::new().template_render(CHARTFILE, &json)
                         .map_err(SyncFailure::new)
                         .map_err(From::from)
    }
}
