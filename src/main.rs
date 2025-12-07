use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

// rcli csv -i input.csv -o output.json --header -d ','

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

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

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "Input CSV file")]
    input: String,

    #[arg(short, long, help = "Output file", default_value = "output.json")]
    output: String,

    #[arg(
        long,
        help = "Indicates that the CSV file has a header row",
        default_value_t = true
    )]
    header: bool,

    #[arg(short, long, help = "Delimiter character", default_value_t = ',')]
    delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file '{}' does not exist.", filename))
    }
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
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
        }
    }

    Ok(())
}
