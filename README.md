# RCLI

`rcli` is a Rust command-line interface (CLI) tool.

## Features

- **CSV Processing**: Convert CSV files to JSON, YAML, or TOML formats.
- **Password Generation**: Generate secure, random passwords with customizable complexity and strength estimation (via zxcvbn).
- **Base64 Encoding/Decoding**: Encode and decode strings using Base64.

## Usage

### Installation

```bash
cargo install --path .
```

1. CSV Conversion

Convert a CSV file to other formats(JSON, YAML, TOML).

```bash
# Convert CSV to JSON
rcli csv --input <input_file.csv> --output <output_file.json> --format json

# Convert CSV to YAML
rcli csv --input <input_file.csv> --output <output_file.yaml> --format yaml
```

2. Password Generation

Generate a random password.

```bash
# Generate a default strong password
rcli genpass

# Generate a password with specific length and rules
rcli genpass --length 16 --no-special --no-numbers
```

3. Base64 Encoding/Decoding

Encode or decode strings using Base64.

```bash
# Encode a string
echo "Hello, World!" | rcli base64 encode

# --format can be 'standard' or 'url_safe'
echo "Hello, World!" | rcli base64 encode --format url_safe
```

```bash
# Decode a Base64 string
echo "SGVsbG8sIFdvcmxkIQ==" | rcli base64 decode

# --format can be 'standard' or 'url_safe'
echo "SGVsbG8sIFdvcmxkIQ==" | rcli base64 decode --format url_safe
```
