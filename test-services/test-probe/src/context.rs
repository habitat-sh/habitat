//! Define routes for exposing the contents of the application's
//! rendering context.

use crate::{config,
            error::Result};
use actix_web::{web,
                HttpRequest,
                HttpResponse};
use serde_json;
use std::{fs,
          path::{Components,
                 Path,
                 PathBuf}};

/// Show render context rooted at `path`
#[allow(clippy::needless_pass_by_value)] // Required by actix-web API
pub fn show(config: web::Data<config::Config>,
            req: HttpRequest)
            -> actix_web::Result<HttpResponse> {
    let path = req.match_info()
                  .get("path")
                  .map(PathBuf::from)
                  .unwrap_or_default();
    let json =
        extract_data(&config.render_context_file, &path).map_err(actix_web::error::ErrorBadRequest)?;

    if let Some(json) = json {
        Ok(HttpResponse::Ok().json(json))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

////////////////////////////////////////////////////////////////////////

fn extract_data<P>(render_context_file: P, path: P) -> Result<Option<serde_json::Value>>
    where P: AsRef<Path>
{
    let data = read_json(render_context_file)?;
    lookup(data, path.as_ref().components())
}

/// Read a file into a JSON value
fn read_json<P>(path: P) -> Result<serde_json::Value>
    where P: AsRef<Path>
{
    let contents = fs::read_to_string(path)?;
    serde_json::from_str(&contents).map_err(|e| e.into())
}

/// Recursively descend through a JSON structure, based on the path
/// provided in `keys`
fn lookup(data: serde_json::Value, keys: Components) -> Result<Option<serde_json::Value>> {
    let mut result = data;
    for component in keys {
        let key: &str = &component.as_os_str().to_string_lossy();

        // For arrays, try to coerce key to an integer and do a lookup
        // that way
        let lookup_result = if result.is_array() {
            result.get(key.parse::<usize>()?)
        } else {
            result.get(key)
        };

        if let Some(stuff) = lookup_result {
            result = stuff.clone(); // eww
        } else {
            return Ok(None);
        }
    }
    Ok(Some(result))
}
