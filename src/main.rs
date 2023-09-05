mod input_generator;
mod tally;

use clap::{Parser, ValueEnum};

use crate::input_generator::generate_input;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    /// Generates random input data for the purpose of tallying
    Generate,

    /// Process tally block voting
    Tally,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Please select a command
    #[arg(value_enum)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate => {
            println!("generate");
            generate_input();
        }
        Command::Tally => {
            println!("tally!");
        }
    }
}
