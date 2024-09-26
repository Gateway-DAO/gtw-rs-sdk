#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! typify = "0.1.0"
//! schemars = "0.8"
//! serde_json = "1.0"
//! reqwest = { version = "0.11", features = ["blocking"] }
//! dotenv = "0.15"
//! ```

use dotenv::dotenv;
use reqwest::blocking;
use schemars::schema::RootSchema;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
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

    let header = "use serde::{Deserialize, Serialize};\n\n";

    let folder_name = "src/types";
    fs::create_dir_all(folder_name)?;

    let file_name = "mod.rs";
    let file_path = Path::new(folder_name).join(file_name);

    let mut file = fs::File::create(&file_path)?;
    file.write_all(header.as_bytes())?;
    file.write_all(rust_defs.as_bytes())?;

    // Format the generated file using rustfmt
    Command::new("rustfmt")
        .arg(file_path)
        .status()
        .expect("Failed to execute rustfmt");

    println!("File created, written, and formatted successfully.");

    Ok(())
}
