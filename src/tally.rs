use fake::{
    faker::{internet::en::SafeEmail, lorem::en::Words, name::en::Name},
    Dummy, Fake, Faker,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contest {
    id: i64,
    description: String,
    tally_type: String,
    num_winners: i64,
    min_choices: i64,
    max_choices: i64,
    choices: Vec<ContestChoice>,
}

impl Dummy<Faker> for Contest {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let id = Fake::fake_with_rng::<i64, _>(&(1..9999), rng);
        let description = Words(3..5).fake::<Vec<String>>().join(" ");
        let tally_type = "plurality-at-large".into();
        let district_magnitude = Fake::fake_with_rng::<i64, _>(&(1..10), rng);
        let choices: Vec<ContestChoice> = (0..district_magnitude).map(|_| Faker.fake()).collect();

        Self {
            id,
            description,
            tally_type,
            num_winners: district_magnitude,
            min_choices: district_magnitude,
            max_choices: district_magnitude,
            choices,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContestChoice {
    id: i64,
    text: String,
    urls: Vec<String>,
}

impl Dummy<Faker> for ContestChoice {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let id = Fake::fake_with_rng::<i64, _>(&(1..9999), rng);
        let text = Fake::fake_with_rng::<String, _>(&(Name()), rng);
        let urls: Vec<String> = (0..rng.gen_range(1..3))
            .map(|_| SafeEmail().fake_with_rng(rng))
            .collect();

        Self { id, text, urls }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedContestVote {
    is_explicit_invalid: bool,
    choices: Vec<DecodedVoteChoice>,
    contest: Contest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedVoteChoice {
    // The choice that was made
    contest_choice: ContestChoice,
    // THe number of votes that were assigned, in plurality at large this is always
    // 0 or 1
    selected: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContestResult {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ContestChoiceResult {
    contest_result: ContestResult,
    contest_choice: ContestChoice,
    total_count: u64,
    // If a winner, the position of this choice (eg 1st, 2nd), otherwise 0
    // Ties are handled by using duplicates, eg 1st, 1st, 3rd..
    winner_position: u64,
}
