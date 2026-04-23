use std::{fs::File,
          io::{Read,
               Write},
          path::{Path,
                 PathBuf}};

pub fn create_with_content<P>(path: P, content: &str)
    where P: AsRef<Path>
{
    let mut file = File::create(path).expect("Cannot create file");
    file.write_all(content.as_bytes())
        .expect("Cannot write to file");
}

pub fn file_content<P>(path: P) -> String
    where P: AsRef<Path>
{
    let mut file = File::open(path).expect("Could not open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");
    contents
}

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
        let pretty_json =
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(json_string).expect("JSON should \
                                                                                  parse if we \
                                                                                  get this far")
                                         ).expect("JSON should serialize if we get this far");
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
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                        .join("fixtures")
                                                        .join(schema);
    let mut schema_file = File::open(path).expect("could not open schema file");
    let mut raw_schema = String::new();
    schema_file.read_to_string(&mut raw_schema)
               .expect("could not read schema file");
    let parsed_schema: serde_json::Value =
        serde_json::from_str(&raw_schema).expect("Could not parse schema as JSON");
    let validator =
        jsonschema::validator_for(&parsed_schema).expect("Could not compile the schema");

    let input_json: serde_json::Value =
        serde_json::from_str(input).expect("Could not parse input as JSON");
    let errors = validator.iter_errors(&input_json)
                          .map(|e| e.to_string())
                          .collect();
    ValidationResult { errors }
}
