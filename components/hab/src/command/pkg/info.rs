use crate::{common::ui::{UIWriter,
                         UI},
            error::Result,
            hcore::{crypto::artifact,
                    package::{PackageArchive,
                              PackageIdent,
                              PackageTarget}}};
use serde::Serialize;
use serde_json::{self,
                 Value as Json};
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct PackageArchiveInfo {
    format_version:    String,
    key_name:          String,
    hash_type:         String,
    signature_raw:     String,
    origin:            String,
    name:              String,
    version:           Option<String>,
    release:           Option<String>,
    checksum:          Option<String>,
    target:            PackageTarget,
    is_a_service:      bool,
    deps:              Vec<PackageIdent>,
    tdeps:             Vec<PackageIdent>,
    build_deps:        Vec<PackageIdent>,
    build_tdeps:       Vec<PackageIdent>,
    exposes:           Vec<u16>,
    pkg_services:      Vec<PackageIdent>,
    resolved_services: Vec<PackageIdent>,
    manifest:          Option<String>,
    config:            Option<String>,
    svc_user:          Option<String>,
    ld_run_path:       Option<String>,
    ldflags:           Option<String>,
    cflags:            Option<String>,
}

fn convert_to_json<T>(src: &T) -> Result<Json>
    where T: Serialize
{
    serde_json::to_value(src).map_err(|e| habitat_core::Error::RenderContextSerialization(e).into())
}

pub fn start(ui: &mut UI, src: &Path, to_json: bool) -> Result<()> {
    let header = artifact::get_artifact_header(src)?;
    let mut archive = PackageArchive::new(src)?;
    let ident = archive.ident()?;
    let info =
        PackageArchiveInfo { format_version:    header.format_version,
                             key_name:          header.key_name,
                             hash_type:         header.hash_type,
                             signature_raw:     header.signature_raw,
                             origin:            ident.origin.clone(),
                             name:              ident.name.clone(),
                             version:           ident.version,
                             release:           ident.release,
                             checksum:          archive.checksum().ok(),
                             target:            archive.target().expect("pkg info archive target"),
                             deps:              archive.deps().unwrap_or_default(),
                             build_deps:        archive.build_deps().unwrap_or_default(),
                             tdeps:             archive.tdeps().unwrap_or_default(),
                             build_tdeps:       archive.build_tdeps().unwrap_or_default(),
                             exposes:           archive.exposes().unwrap_or_default(),
                             pkg_services:      archive.pkg_services().unwrap_or_default(),
                             resolved_services: archive.resolved_services().unwrap_or_default(),
                             manifest:
                                 archive.manifest()
                                        .map_or_else(|_| None, |v| Some(v.to_string())),
                             config:            archive.config().map(|v| v.to_string()),
                             svc_user:
                                 archive.svc_user()
                                        .map_or_else(|_| None, |v| Some(v.to_string())),
                             ld_run_path:       archive.ld_run_path().map(|v| v.to_string()),
                             ldflags:           archive.ldflags().map(|v| v.to_string()),
                             cflags:            archive.cflags().map(|v| v.to_string()),
                             is_a_service:      archive.is_a_service(), };

    if to_json {
        match convert_to_json(&info) {
            Ok(content) => {
                println!("{}", content);
                return Ok(());
            }
            Err(e) => {
                ui.fatal(format!("Failed to deserialize into json! {:?}.", e))?;
                return Err(e);
            }
        }
    } else {
        ui.begin(format!("Reading PackageIdent from {}", &src.display()))?;
        ui.para("")?;

        println!("Package Path   : {}", &src.display());
        println!("Origin         : {}", info.origin);
        println!("Name           : {}", info.name);
        println!("Version        : {}",
                 info.version.unwrap_or_else(|| "".to_string()));
        println!("Release        : {}",
                 info.release.unwrap_or_else(|| "".to_string()));
    }
    Ok(())
}
