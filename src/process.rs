use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::opts::CsvOpts;
use anyhow::Ok;

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
    println!("Processing CSV file: {}", opts.input);
    let mut reader_builder = Reader::from_path(&opts.input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader_builder.deserialize() {
        let player: Player = result?;
        // println!("{:?}", player);
        ret.push(player);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(&opts.output, json)?;

    Ok(())
}
