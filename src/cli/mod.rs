mod base64_opts;
mod csv_opts;
mod genpass_opts;

use std::path::Path;

use clap::Parser;

pub use self::{
    base64_opts::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand},
    csv_opts::{CsvOpts, OutputFormat},
    genpass_opts::GenPassOpts,
};

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

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand, name = "base64", about = "Encode or decode Base64 data")]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file '{}' does not exist.", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        // Test with existing file
        let existing_file = "src/lib.rs"; // Assuming this file exists
        assert!(verify_input_file(existing_file).is_ok());
        // Test with non-existing file
        let non_existing_file = "non_existing_file.txt";
        assert!(verify_input_file(non_existing_file).is_err());
        // Test with "-" (stdin)
        let stdin_input = "-";
        assert!(verify_input_file(stdin_input).is_ok());
    }
}
