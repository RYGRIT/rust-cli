use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, help = "Length of the password", default_value_t = 12)]
    pub length: u8,

    #[arg(long = "no-uppercase", help = "Exclude uppercase characters", default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub uppercase: bool,

    #[arg(long = "no-lowercase", help = "Exclude lowercase characters", default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub lowercase: bool,

    #[arg(long = "no-numbers", help = "Exclude numbers", default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub numbers: bool,

    #[arg(long = "no-special", help = "Exclude special characters", default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub special: bool,
}
