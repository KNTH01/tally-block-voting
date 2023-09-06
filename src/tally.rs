use serde::Deserialize;
use std::collections::HashMap;

use std::fs;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::voting::{
    Contest, ContestChoiceResult, ContestResult, DecodedContestVote, DecodedVoteChoice,
};

pub fn process_tally(input: PathBuf, output: PathBuf) {
    let input_json = read_input(input);

    let cr = count_votes(&input_json);

    let json_data = serde_json::to_string(&cr).expect("Failed to serialize contest result");

    let filename = output.to_str().expect("Failed to read output file");
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
        let choice = contest
            .choices
            .iter()
            .find(|choice| choice.id == *choice_id)
            .cloned();

        if let Some(choice) = choice {
            results.push(ContestChoiceResult {
                contest_choice: choice,
                total_count: *count,
                winner_position: 0, // init winner_position to 0
            });
        }
    }

    if total_valid_votes == 0 {
        panic!("There is no valid votes");
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
            position += 2;
            prev_max_count = results[i].total_count;
        }

        results[i].winner_position = position - 1;
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
    votes: Vec<DecodedContestVoteId>,
}

#[derive(Deserialize, Debug)]
pub struct DecodedContestVoteId {
    is_explicit_invalid: bool,
    choices: Vec<DecodedVoteChoiceId>,
}

#[derive(Deserialize, Debug)]
struct DecodedVoteChoiceId {
    contest_choice: i64,
    selected: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data_generator::generate_input, voting::ContestChoice};
    use std::{path::Path, vec};

    #[test]
    #[should_panic]
    fn test_read_invalid_input_file() {
        let path = Path::new("test-abc.json");
        read_input(path.to_path_buf());
    }

    #[test]
    fn test_read_input_output_files() {
        let input = Path::new("testing-input.json");
        let output = Path::new("testing-output.json");

        generate_input(input.to_path_buf());
        assert!(input.exists());

        process_tally(input.to_path_buf(), output.to_path_buf());
        assert!(output.exists());
    }

    #[test]
    fn test_count_votes() {
        let district_magnitude = 3;

        let choices: Vec<ContestChoice> = (1..=6)
            .map(|i| ContestChoice {
                id: i,
                text: i.to_string(),
                urls: vec![],
            })
            .collect();

        let contest = Contest {
            id: 1,
            description: "Bonjour !".into(),
            tally_type: "plurality-at-large".into(),
            num_winners: district_magnitude,
            min_choices: district_magnitude,
            max_choices: district_magnitude,
            choices,
        };

        let vote1 = DecodedContestVoteId {
            is_explicit_invalid: false,
            choices: vec![
                DecodedVoteChoiceId {
                    contest_choice: 1,
                    selected: 1,
                },
                DecodedVoteChoiceId {
                    contest_choice: 2,
                    selected: 1,
                },
                DecodedVoteChoiceId {
                    contest_choice: 3,
                    selected: 1,
                },
            ],
        };

        let vote2 = DecodedContestVoteId {
            is_explicit_invalid: false,
            choices: vec![
                DecodedVoteChoiceId {
                    contest_choice: 1,
                    selected: 1,
                },
                DecodedVoteChoiceId {
                    contest_choice: 2,
                    selected: 1,
                },
                DecodedVoteChoiceId {
                    contest_choice: 4,
                    selected: 1,
                },
            ],
        };

        let vote3 = DecodedContestVoteId {
            is_explicit_invalid: false,
            choices: vec![
                DecodedVoteChoiceId {
                    contest_choice: 1,
                    selected: 1,
                },
                DecodedVoteChoiceId {
                    contest_choice: 2,
                    selected: 1,
                },
                DecodedVoteChoiceId {
                    contest_choice: 4,
                    selected: 1,
                },
            ],
        };

        let votes = vec![vote1, vote2, vote3];

        let input = InputJson {
            contest: contest.clone(),
            votes,
        };

        let cr = count_votes(&input);

        assert_eq!(cr.contest.id, contest.id);
        assert_eq!(cr.total_valid_votes, 3);
        assert_eq!(cr.total_invalid_votes, 0);

        // assert winner contest_choice
        let winner_ids = vec![1, 2, 4];
        for w in &cr.winners {
            assert!(winner_ids.iter().any(|id| *id == w.id));
        }

        println!(
            "{:?}",
            cr.results
                .iter()
                .map(|res| res.winner_position)
                .collect::<Vec<u64>>()
        );

        // assert winner_position
        assert_eq!(
            cr.results
                .iter()
                .take(3)
                .map(|res| res.winner_position)
                .collect::<Vec<u64>>(),
            vec![1, 1, 3]
        );
    }
}
