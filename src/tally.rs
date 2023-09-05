use serde::Deserialize;

use std::{fs::File, io::BufReader, path::PathBuf};

use crate::voting::{Contest, DecodedVoteChoice};

pub fn process_tally(file: PathBuf) {
    let input = read_input(file);



}

fn read_input(file: PathBuf) -> InputJson {
    let filename = file.to_str().unwrap();
    let file = File::open(filename).expect("Unable to open file {filename}");
    let reader = BufReader::new(file);
    let input: InputJson = serde_json::from_reader(reader).unwrap();

    println!("{:?}", &input);

    input
}

#[derive(Deserialize, Debug)]
struct InputJson {
    contest: Contest,
    votes: Vec<DecodedVoteChoiceId>,
}

#[derive(Deserialize, Debug)]
pub struct DecodedVoteChoiceId {
    is_explicit_invalid: bool,
    choices: Vec<DecodedContestVoteId>,
    contest: i64,
}

#[derive(Deserialize, Debug)]
struct DecodedContestVoteId {
    contest_choice: i64, // holds ids
    selected: u64,
}
