#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! typify = "0.1.0"
//! schemars = "0.8"
//! serde_json = "1.0"
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```

use reqwest::blocking;
use schemars::schema::RootSchema;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_url = "https://dev.api.gateway.tech/swagger/doc.json";

    // Fetch the schema from the URL
    let response = blocking::get(schema_url)?;
    let schema_str = response.text()?;

    // Parse the schema as JSON
    let schema_json: Value = serde_json::from_str(&schema_str)?;
    let root_schema: RootSchema = serde_json::from_value(schema_json)?;
    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space.add_root_schema(root_schema)?;

    // Generate and print Rust type definitions
    let rust_defs = type_space.to_stream().to_string();
    let folder_name = "src/models";
    let file_name = "generated_models.rs";
    let file_path = Path::new(folder_name).join(file_name);
    let mut file = fs::File::create(file_path)?;
    file.write_all(rust_defs.as_bytes())?;

    println!("File created and written to successfully.");

    Ok(())
}
