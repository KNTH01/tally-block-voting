use crate::voting::{Contest, DecodedContestVote};
use fake::{Fake, Faker};
use rand::Rng;
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Serialize)]
struct InputJson {
    contest: Contest,
    votes: Vec<DecodedContestVote>,
}

pub fn generate_input(file: PathBuf) {
    let contest = generate_contest();
    let votes = generate_votes(&contest, contest.get_district_magnitude().unwrap());

    let input = InputJson { contest, votes };
    let json_data = serde_json::to_string(&input).expect("Failed to serialize contest");

    let filename = file.to_str().unwrap();
    fs::write(filename, json_data).expect("Unable to write input data into file");

    println!("Generated {filename}")
}

fn generate_contest() -> Contest {
    Faker.fake()
}

fn generate_votes(contest: &Contest, district_magnitude: u64) -> Vec<DecodedContestVote> {
    let mut rng = rand::thread_rng();

    (100..rng.gen_range(200..500))
        .map(|_| DecodedContestVote::dummy(contest.clone(), district_magnitude))
        .collect()
}
