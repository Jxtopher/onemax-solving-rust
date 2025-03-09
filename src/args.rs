use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 0)]
    pub seed: u64,

    /// Bitstring size
    #[arg(short, default_value_t = 100)]
    pub n: usize,

    /// Rounds
    #[arg(short, default_value_t = u64::MAX)]
    pub rounds: u64,

    /// Configuration
    #[arg(short, default_value_t = ("").to_string())]
    pub configuration: String,
}
