use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, SubCommand, process_csv, process_decode, process_encode,
    process_genpass,
};

// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(&opts)?;
        }
        SubCommand::Base64(base64) => match base64 {
            Base64SubCommand::Encode(opts) => process_encode(&opts)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts)?,
        },
    }

    Ok(())
}
