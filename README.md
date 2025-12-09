# RCLI

`rcli` is a Rust command-line interface (CLI) tool.

## Features

- **CSV Processing**: Convert CSV files to JSON, YAML, or TOML formats.
- **Password Generation**: Generate secure, random passwords with customizable complexity and strength estimation (via zxcvbn).

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
