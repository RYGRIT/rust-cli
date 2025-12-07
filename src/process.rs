use std::{collections::HashMap, fs};

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::{CsvOpts, OutputFormat};
use anyhow::Ok;

const TOML_ROOT_KEY: &str = "data";

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(opts: &CsvOpts) -> anyhow::Result<()> {
    let output = match &opts.output {
        Some(output) => output.clone(),
        None => format!("output.{}", opts.format),
    };
    println!(
        "Processing CSV file: {}, output file: {}",
        opts.input, output
    );
    let mut reader_builder = Reader::from_path(&opts.input)?;
    let mut parsed_records = Vec::with_capacity(128);
    let headers = reader_builder.headers()?.clone();
    for result in reader_builder.records() {
        let record = result?;
        let json_value = serde_json::Value::Object(
            headers
                .iter()
                // zip() the headers and record into a JSON object
                .zip(record.iter())
                .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
                .collect(),
        );
        parsed_records.push(json_value);
    }

    let content = match opts.format {
        OutputFormat::Json => serde_json::to_string_pretty(&parsed_records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&parsed_records)?,
        // TOML format requires the root element
        OutputFormat::Toml => {
            let mut map = HashMap::new();
            map.insert(TOML_ROOT_KEY, &parsed_records);
            toml::to_string(&map)?
        }
    };

    fs::write(output, content)?;
    println!("File converted successfully.");

    Ok(())
}
