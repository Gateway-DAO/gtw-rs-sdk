#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! typify = "0.1.0"
//! schemars = "0.8"
//! serde_json = "1.0"
//! reqwest = { version = "0.11", features = ["blocking"] }
//! dotenv = "0.15"
//! rustfmt-wrapper = "0.2.1"
//! ```

use dotenv::dotenv;
use reqwest::blocking;
use schemars::schema::RootSchema;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let schema_url = env::var("SCHEMA_URL").expect("SCHEMA_URL is not set");

    // Fetch the schema from the URL
    let response = blocking::get(&schema_url)?;
    let schema_str = response.text()?;

    let schema_json: Value = serde_json::from_str(&schema_str)?;
    let root_schema: RootSchema = serde_json::from_value(schema_json)?;
    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space.add_root_schema(root_schema)?;

    let rust_defs = type_space.to_stream().to_string();

    let header = "// Code generated using `make generate`.\n// Don't change it\n\nuse serde::{Deserialize, Serialize};\n\n";

    let content = rustfmt_wrapper::rustfmt(format!("{}{}", header, rust_defs))?;

    let folder_name = "src/models";
    fs::create_dir_all(folder_name)?;

    let file_name = "mod.rs";
    let file_path = Path::new(folder_name).join(file_name);

    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    // Get all filenames in the folder except mod.rs
    let files = fs::read_dir(folder_name)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    // Add all public modules to the mod.rs file
    let mut mod_file_content = String::new();
    for file in files {
        let file_name = file.to_str().unwrap();
        if file_name != "mod.rs" {
            mod_file_content.push_str(&format!("pub mod {};\n", file_name.replace(".rs", "")));
        }
    }
    file.write_all(mod_file_content.as_bytes())?;

    println!("File created and written to successfully.");

    Ok(())
}
