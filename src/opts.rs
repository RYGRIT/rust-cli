use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "Input CSV file")]
    pub input: String,

    #[arg(short, long, help = "Output file", default_value = "output.json")]
    pub output: String,

    #[arg(
        long,
        help = "Indicates that the CSV file has a header row",
        default_value_t = true
    )]
    pub header: bool,

    #[arg(short, long, help = "Delimiter character", default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file '{}' does not exist.", filename))
    }
}
