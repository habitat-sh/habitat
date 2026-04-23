use std::{fs,
          path::{Path,
                 PathBuf}};

/// The result of validating a JSON document against a schema.
pub struct ValidationResult {
    pub errors: Vec<String>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool { self.errors.is_empty() }
}

/// Asserts that `json_string` is valid according to the specified JSON schema.
///
/// In the case of invalid input, all validation errors are neatly
/// rendered, along with a pretty-printed formatting of the
/// offending JSON.
pub fn assert_valid(json_string: &str, schema: &str) {
    let result = validate_string(json_string, schema);
    if !result.is_valid() {
        let error_string = result.errors
                                 .iter()
                                 .map(|x| format!("  {}", x))
                                 .collect::<Vec<String>>()
                                 .join("\n");
        let pretty_json = serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(json_string).expect("JSON should parse if we get this far")).expect("JSON should serialize if we get this far");
        panic!(
               r#"
JSON does not validate!
Errors:
{}

JSON:
{}
"#,
               error_string, pretty_json
        );
    }
}

/// Compares the incoming JSON string against the JSON schema
/// and returns the resulting `ValidationResult`.
///
/// In general, you should prefer using `assert_valid` directly.
pub fn validate_string(input: &str, schema: &str) -> ValidationResult {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("doc")
                                                        .join(schema);
    let validator = build_validator(&path);
    let input_json: serde_json::Value =
        serde_json::from_str(input).expect("Could not parse input as JSON");
    let errors = validator.iter_errors(&input_json)
                          .map(|e| e.to_string())
                          .collect();
    ValidationResult { errors }
}

/// Build a `jsonschema` validator for the schema at `path`.
///
/// Schemas in this repository cross-reference sibling files (e.g.
/// `render_context_schema.json#/definitions/package_identifier`).  The
/// `referencing` crate used internally by jsonschema does not ship a working
/// file-system retriever, so we pre-process the root schema instead:
///
/// 1. Merge the `definitions` sections of every sibling `.json` file into the root schema's own
///    `definitions`, keeping existing entries intact.
/// 2. Rewrite every cross-schema `$ref` of the form `other.json#/ptr` to the purely-local form
///    `#/ptr`.
///
/// After these two steps all `$ref`s are self-contained and `jsonschema` can
/// compile the schema without needing an external retriever.
fn build_validator(path: &Path) -> jsonschema::Validator {
    let schema_dir = path.parent().expect("schema file has no parent directory");

    let mut root: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(path).expect("could not read schema file"))
            .expect("Could not parse schema as JSON");

    // 1 — merge sibling definitions
    for entry in fs::read_dir(schema_dir).expect("could not read schema directory") {
        let sibling_path = entry.expect("could not read schema directory entry").path();
        if sibling_path.extension().and_then(|e| e.to_str()) != Some("json") || sibling_path == path
        {
            continue;
        }
        let sibling: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(&sibling_path).expect("could not read sibling schema"),
        )
        .expect("could not parse sibling schema as JSON");

        if let Some(sibling_defs) = sibling.get("definitions").and_then(|v| v.as_object()) {
            let root_obj = root.as_object_mut()
                               .expect("schema root must be a JSON object");
            let root_defs =
                root_obj.entry("definitions")
                        .or_insert_with(|| serde_json::Value::Object(Default::default()));
            let root_defs_map = root_defs.as_object_mut()
                                         .expect("'definitions' must be a JSON object");
            for (key, value) in sibling_defs {
                root_defs_map.entry(key.clone())
                             .or_insert_with(|| value.clone());
            }
        }
    }

    // 2 — rewrite cross-schema refs to local refs
    rewrite_cross_schema_refs(&mut root);

    jsonschema::validator_for(&root).expect("Could not compile the schema")
}

/// Recursively rewrite every `$ref` of the form `"other.json#/pointer"` to the
/// purely-local form `"#/pointer"`, leaving local refs (those that already
/// start with `#`) unchanged.
fn rewrite_cross_schema_refs(val: &mut serde_json::Value) {
    match val {
        serde_json::Value::Object(obj) => {
            if let Some(ref_val) = obj.get("$ref").and_then(|v| v.as_str()).map(str::to_owned) {
                if let Some((file, ptr)) = ref_val.split_once('#') {
                    if !file.is_empty() {
                        obj.insert("$ref".to_string(),
                                   serde_json::Value::String(format!("#{ptr}")));
                    }
                }
            }
            for v in obj.values_mut() {
                rewrite_cross_schema_refs(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                rewrite_cross_schema_refs(v);
            }
        }
        _ => {}
    }
}
