//! Shared by the schema-backed integration tests.

use std::fs;

/// Read a RELAX NG schema from the gitignored `spec/` download.
///
/// Returns `None`, with a note on stderr, when the schemas have not been
/// downloaded, so the tests that need them pass vacuously on a checkout
/// that has not run `scripts/download-spec.sh`.
pub fn read_schema(name: &str) -> Option<String> {
    let path = format!(
        "{}/../../spec/OfficeOpenXML-RELAXNG-Transitional/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    match fs::read_to_string(&path) {
        Ok(input) => Some(input),
        Err(_) => {
            eprintln!(
                "skipping: {path} not found; run scripts/download-spec.sh and extract the RELAX NG schemas"
            );
            None
        }
    }
}
