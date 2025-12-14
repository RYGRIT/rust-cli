use core::fmt;
use std::str::FromStr;

use clap::Parser;

use crate::cli::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode data to Base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode data from Base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, help = "Input file path or '-' to encode from stdin", value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(
        short,
        long,
        value_parser = parse_base64_format,
        help = "Base64 format (standard or url-safe)",
        default_value = "standard"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, help = "Input file path or '-' to decode from stdin", value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(
        short,
        long,
        value_parser = parse_base64_format,
        help = "Base64 format (standard or url-safe)",
        default_value = "standard"
    )]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "url-safe" | "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Unsupported Base64 format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url-safe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url-safe",
        };
        write!(f, "{}", s)
    }
}
