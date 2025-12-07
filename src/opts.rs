use std::fmt;
use std::{path::Path, str::FromStr};

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

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl From<&OutputFormat> for &'static str {
    fn from(format: &OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Unsupported output format: {}", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = self.into();
        write!(f, "{}", s)
    }
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file, help = "Input CSV file")]
    pub input: String,

    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format, help = "Output format", default_value = "json")]
    pub format: OutputFormat,

    #[arg(
        long,
        help = "Indicates that the CSV file has a header row",
        default_value_t = true
    )]
    pub header: bool,

    #[arg(short, long, help = "Delimiter character", default_value_t = ',')]
    pub delimiter: char,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, help = "Length of the password", default_value_t = 12)]
    pub length: u8,

    #[arg(long, help = "Include uppercase characters", default_value_t = true, action = clap::ArgAction::Set)]
    pub uppercase: bool,

    #[arg(long, help = "Include lowercase characters", default_value_t = true, action = clap::ArgAction::Set)]
    pub lowercase: bool,

    #[arg(long, help = "Include numbers", default_value_t = true, action = clap::ArgAction::Set)]
    pub numbers: bool,

    #[arg(long, help = "Include special characters", default_value_t = false, action = clap::ArgAction::Set)]
    pub special: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file '{}' does not exist.", filename))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
