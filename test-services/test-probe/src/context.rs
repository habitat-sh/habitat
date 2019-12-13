//! Define routes for exposing the contents of the application's
//! rendering context.

use crate::{config,
            error::Result};
use actix_web::{web,
                HttpRequest,
                HttpResponse};
use serde_json;
use std::{borrow::Cow,
          fs,
          path::Path};

/// Show render context rooted at `path`
#[allow(clippy::needless_pass_by_value)] // Required by actix-web API
pub fn show(config: web::Data<config::Config>,
            req: HttpRequest)
            -> actix_web::Result<HttpResponse> {
    // A JSON pointer of "" means "everything". Pointers starting with
    // "/" describe paths into the structure. We'll chop off any
    // trailing "/" since that will end up looking for an item with a
    // key of "", which we're just not going to deal with.
    let pointer = match req.match_info().get("path") {
        None | Some("") => Cow::Borrowed(""),
        Some(path) => Cow::Owned(format!("/{}", path.trim_end_matches('/'))),
    };
    let json = read_json(&config.render_context_file).map_err(actix_web::error::ErrorBadRequest)?;
    if let Some(value) = json.pointer(&pointer) {
        Ok(HttpResponse::Ok().json(value))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

////////////////////////////////////////////////////////////////////////

/// Read a file into a JSON value
fn read_json<P>(path: P) -> Result<serde_json::Value>
    where P: AsRef<Path>
{
    let contents = fs::read_to_string(path)?;
    serde_json::from_str(&contents).map_err(|e| e.into())
}
