#![allow(dead_code)]

use clap::{Parser, ValueEnum};

struct Contest {
    id: i64,
    description: String,
    // Will be "plurality-at-large"
    tally_type: String,
    // The number of choices that will be "elected", see ContestChoiceResult::winner_position
    num_winners: i64,
    min_choices: i64,
    max_choices: i64,
    choices: Vec<ContestChoice>,
}

struct ContestChoice {
    id: i64,
    text: String,
    urls: Vec<String>,
}

struct DecodedContestVote {
    is_explicit_invalid: bool,
    choices: Vec<DecodedVoteChoice>,
    contest: Contest,
}

struct DecodedVoteChoice {
    // The choice that was made
    contest_choice: ContestChoice,
    // THe number of votes that were assigned, in plurality at large this is always
    // 0 or 1
    selected: u64,
}

struct ContestResult {
    contest: Contest,
    total_valid_votes: i64,
    // For this exercise a vote is invalid if:
    // DecodedContestVote::is_explicit_invalid is set to true, or
    // The number of selected choices does not comply with Contest::min/max_choices
    total_invalid_votes: i64,
    // The counts per choice
    results: Vec<ContestChoiceResult>,
    // The winners for the contest (see Contest:num_winners)
    winners: Vec<ContestChoice>,
}

struct ContestChoiceResult {
    contest_result: ContestResult,
    contest_choice: ContestChoice,
    total_count: u64,
    // If a winner, the position of this choice (eg 1st, 2nd), otherwise 0
    // Ties are handled by using duplicates, eg 1st, 1st, 3rd..
    winner_position: u64,
}

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
            println!("generate")
        }
        Command::Tally => {
            println!("tally!");
        }
    }
}
