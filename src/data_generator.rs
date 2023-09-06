use crate::voting::{Contest, DecodedContestVote};
use fake::{Fake, Faker};
use mockall::automock;
use rand::Rng;
use serde::Serialize;
use std::{fs, path::PathBuf};

pub fn generate_input(file: PathBuf) {
    let generator = DataGenerator {};
    let contest = generator.generate_contest();
    let votes = generator
        .generate_votes(&contest, contest.get_district_magnitude()
        .expect("This program only implement plurality-at-large vote, so it needs to get a district magnitude"));

    let input = InputJson { contest, votes };
    let json_data = serde_json::to_string(&input).expect("Failed to serialize contest");

    let filename = file.to_str().unwrap();
    fs::write(filename, json_data).expect("Unable to write input data into file");

    println!("Generated {filename}")
}

#[derive(Serialize, Debug)]
struct InputJson {
    contest: Contest,
    votes: Vec<DecodedContestVote>,
}

pub struct DataGenerator {}

#[automock]
pub trait UseDataGenerator {
    fn generate_contest(&self) -> Contest;
    fn generate_votes(&self, contest: &Contest, district_magnitude: u64)
        -> Vec<DecodedContestVote>;
}

impl UseDataGenerator for DataGenerator {
    fn generate_contest(&self) -> Contest {
        Faker.fake()
    }

    fn generate_votes(
        &self,
        contest: &Contest,
        district_magnitude: u64,
    ) -> Vec<DecodedContestVote> {
        let mut rng = rand::thread_rng();

        (100..rng.gen_range(200..500))
            .map(|_| DecodedContestVote::dummy(contest.clone(), district_magnitude))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::voting::ContestChoice;

    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_generate_input_file() {
        let path = Path::new("test_output.json");
        generate_input(path.to_path_buf());
        assert!(path.exists());
        let data = fs::read_to_string(path).expect("Unable to read file");
        assert!(!data.is_empty());
        fs::remove_file(path).expect("Failed to clean up test file");
    }

    #[test]
    fn test_generate_input_data() {
        let mut mock = MockUseDataGenerator::new();

        let district_magnitude = 3;
        let choices: Vec<ContestChoice> = Faker.fake();
        let contest = Contest {
            id: 1,
            description: "Bonjour !".into(),
            tally_type: "plurality-at-large".into(),
            num_winners: district_magnitude,
            min_choices: district_magnitude,
            max_choices: district_magnitude,
            choices,
        };

        let votes: Vec<DecodedContestVote> = (1..10)
            .map(|_| DecodedContestVote::dummy(contest.clone(), district_magnitude as u64))
            .collect();

        mock.expect_generate_contest()
            .returning(move || contest.clone());

        mock.expect_generate_votes()
            .returning(move |_, _| votes.clone());
    }
}
