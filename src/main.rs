use clap::Parser;
use rcli::{Opts, SubCommand, process_csv, process_genpass};

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
    }

    Ok(())
}
