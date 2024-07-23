use std::{env, fs::File, io::Write};

use cassette_core::document::Document;
use include_dir::{include_dir, Dir};

const OUT_SRC: &str = "examples.yaml";

fn main() {
    static EXAMPLES: Dir<'_> = include_dir!("examples");
    println!("cargo:rerun-if-changed=../../examples");

    let contents = EXAMPLES
        .files()
        .filter(|file| file.path().to_string_lossy().ends_with(".yaml"))
        .filter_map(|file| file.contents_utf8())
        .flat_map(|docs| {
            docs.split("---\n")
                .map(|doc| doc.trim())
                .filter(|doc| !doc.is_empty())
        });

    // YAML -> Documents
    let documents: Vec<Document> = contents
        .map(|doc| ::serde_yml::from_str(doc).expect("failed to parse example resource file"))
        .collect();

    // Documents -> JSON
    let documents_str =
        ::serde_json::to_string(&documents).expect("failed to serialize example resources to JSON");

    let out_dir = env::var("OUT_DIR").expect("cannot parse `OUT_DIR` environment variable");
    let mut dst_file = File::create(format!("{out_dir}/{OUT_SRC}"))
        .expect("failed to create example resource dst file");

    dst_file
        .write_all(documents_str.as_bytes())
        .expect("failed to write example resources");
    dst_file.flush().expect("failed to flush example resources");
}
