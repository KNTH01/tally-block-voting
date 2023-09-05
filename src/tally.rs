use serde::Deserialize;
use std::collections::HashMap;

use std::fs;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::voting::{
    Contest, ContestChoice, ContestChoiceResult, ContestResult, DecodedContestVote,
    DecodedVoteChoice,
};

pub fn process_tally(file: PathBuf) {
    let input = read_input(file);

    let cr = count_votes(&input);

    let json_data = serde_json::to_string(&cr).expect("Failed to serialize contest result");

    let filename = "output.json";
    fs::write(filename, json_data).expect("Unable to write input data into file");

    println!("Generated {filename}");
}

fn read_input(file: PathBuf) -> InputJson {
    let filename = file.to_str().unwrap();
    let file = File::open(filename).expect("Unable to open file {filename}");
    let reader = BufReader::new(file);
    let input: InputJson = serde_json::from_reader(reader).unwrap();

    input
}

fn count_votes(input: &InputJson) -> ContestResult {
    let contest = input.contest.clone();
    let votes: Vec<DecodedContestVote> = input
        .votes
        .iter()
        .map(|vote| {
            let choices = vote
                .choices
                .iter()
                .map(|choice| {
                    DecodedVoteChoice::get_vote_choice_by_id(
                        choice.contest_choice,
                        contest.choices.clone(),
                        choice.selected,
                    )
                })
                .collect();

            DecodedContestVote {
                is_explicit_invalid: vote.is_explicit_invalid,
                choices,
                contest: contest.clone(),
            }
        })
        .collect();

    create_contest_results(votes, &contest)
}

fn create_contest_results(votes: Vec<DecodedContestVote>, contest: &Contest) -> ContestResult {
    let mut choice_count: HashMap<i64, u64> = HashMap::new();

    let mut total_valid_votes = 0;
    let mut total_invalid_votes = 0;

    for vote in votes {
        if vote.is_explicit_invalid
            || vote.choices.len() < contest.min_choices as usize
            || vote.choices.len() > contest.max_choices as usize
        {
            total_invalid_votes += 1;
        } else {
            total_valid_votes += 1;
            for choice in vote.choices {
                *choice_count.entry(choice.contest_choice.id).or_insert(0) += choice.selected;
            }
        }
    }

    let mut results: Vec<ContestChoiceResult> = vec![];
    for (choice_id, count) in &choice_count {
        let choice: ContestChoice = contest
            .choices
            .iter()
            .find(|choice| choice.id == *choice_id)
            .cloned()
            .unwrap();

        results.push(ContestChoiceResult {
            contest_choice: choice,
            total_count: *count,
            winner_position: 0, // init winner_position to 0
        });
    }

    // sort results with the biggest amount of votes
    results.sort_by(|a, b| b.total_count.cmp(&a.total_count));

    let district_magnitude = std::cmp::min(contest.num_winners as usize, results.len());

    let winners = results
        .iter()
        .take(district_magnitude)
        .cloned()
        .collect::<Vec<_>>();

    let mut prev_max_count = results[0].total_count + 1;
    let mut position = 0;
    for (i, _winner) in winners.iter().enumerate() {
        if results[i].total_count != prev_max_count {
            position += 1;
            prev_max_count = results[i].total_count;
        }
        results[i].winner_position = position;
    }

    ContestResult {
        contest: contest.clone(),
        total_valid_votes,
        total_invalid_votes,
        results,
        winners: winners.into_iter().map(|res| res.contest_choice).collect(),
    }
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
}

#[derive(Deserialize, Debug)]
struct DecodedContestVoteId {
    contest_choice: i64, // holds ids
    selected: u64,
}
