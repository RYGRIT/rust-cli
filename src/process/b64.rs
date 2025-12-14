use std::{
    fs::File,
    io::{self, Read, Write},
};

use anyhow::Result;
use base64::prelude::*;

use crate::cli::{Base64DecodeOpts, Base64EncodeOpts, Base64Format};

pub fn process_encode(opts: &Base64EncodeOpts) -> Result<()> {
    let data = get_input(&opts.input)?;

    let encoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.encode(&data),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&data),
    };

    println!("{encoded}");
    Ok(())
}

pub fn process_decode(opts: &Base64DecodeOpts) -> Result<()> {
    let data = get_input(&opts.input)?;

    // avoid accidental newline/spaces in base64 text input
    let data = data
        .into_iter()
        .filter(|b| !b.is_ascii_whitespace())
        .collect::<Vec<u8>>();

    let decoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.decode(&data)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(&data)?,
    };

    // Base64 can represent arbitrary binary data; write raw bytes to stdout.
    let mut stdout = io::stdout().lock();
    stdout.write_all(&decoded)?;
    Ok(())
}

fn get_input(input: &str) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "fixtures/b64.txt";
        let content = "Hello, World!";
        std::fs::write(input, content).unwrap();

        let opts = Base64EncodeOpts {
            input: input.to_string(),
            format: Base64Format::Standard,
        };
        assert!(process_encode(&opts).is_ok());

        let opts = Base64EncodeOpts {
            input: input.to_string(),
            format: Base64Format::UrlSafe,
        };
        assert!(process_encode(&opts).is_ok());

        let _ = std::fs::remove_file(input);
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64_decode.txt";
        let content = "SGVsbG8sIFdvcmxkIQ==";
        std::fs::write(input, content).unwrap();

        let opts = Base64DecodeOpts {
            input: input.to_string(),
            format: Base64Format::Standard,
        };
        assert!(process_decode(&opts).is_ok());

        // Test UrlSafe (without padding)
        let content_no_pad = "SGVsbG8sIFdvcmxkIQ";
        std::fs::write(input, content_no_pad).unwrap();
        let opts = Base64DecodeOpts {
            input: input.to_string(),
            format: Base64Format::UrlSafe,
        };
        assert!(process_decode(&opts).is_ok());

        let _ = std::fs::remove_file(input);
    }
}
