use fake::{
    faker::{internet::en::SafeEmail, lorem::en::Words, name::en::Name},
    Dummy, Fake, Faker,
};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contest {
    pub id: i64,
    pub description: String,
    pub tally_type: String,
    pub num_winners: i64,
    pub min_choices: i64,
    pub max_choices: i64,
    pub choices: Vec<ContestChoice>,
}

impl Dummy<Faker> for Contest {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let id = Fake::fake_with_rng::<i64, _>(&(1..9999), rng);
        let description = Words(3..5).fake::<Vec<String>>().join(" ");
        let tally_type = "plurality-at-large".into();
        let district_magnitude = Fake::fake_with_rng::<i64, _>(&(1..5), rng);
        let choices: Vec<ContestChoice> = (district_magnitude..50).map(|_| Faker.fake()).collect();

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

impl Contest {
    pub fn get_district_magnitude(&self) -> Option<u64> {
        if self.tally_type == "plurality-at-large"
            && (self.num_winners == self.min_choices && self.num_winners == self.max_choices)
        {
            return Some(self.num_winners as u64);
        }

        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContestChoice {
    pub id: i64,
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

#[derive(Serialize, Clone, Debug)]
pub struct DecodedContestVote {
    pub is_explicit_invalid: bool,
    pub choices: Vec<DecodedVoteChoice>,
    #[serde(serialize_with = "DecodedContestVote::to_id")]
    pub contest: Contest,
}

impl DecodedContestVote {
    pub fn dummy(contest: Contest, district_magnitude: u64) -> Self {
        let mut rng = rand::thread_rng();

        let choices: Vec<_> = contest
            .choices
            .choose_multiple(&mut rng, district_magnitude as usize)
            .map(|choice| DecodedVoteChoice {
                contest_choice: choice.clone(),
                selected: 1,
            })
            .collect();

        Self {
            is_explicit_invalid: false,
            choices,
            contest,
        }
    }

    pub fn to_id<S>(contest: &Contest, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(contest.id)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecodedVoteChoice {
    #[serde(serialize_with = "DecodedVoteChoice::to_id")]
    // The choice that was made
    pub contest_choice: ContestChoice,
    // The number of votes that were assigned, in plurality at large this is always
    // 0 or 1
    pub selected: u64,
}

impl DecodedVoteChoice {
    pub fn get_vote_choice_by_id(
        id: i64,
        contest_choices: Vec<ContestChoice>,
        selected: u64,
    ) -> Self {
        let contest_choice: ContestChoice = contest_choices
            .iter()
            .find(|cc| cc.id == id)
            .unwrap()
            .clone();

        Self {
            contest_choice,
            selected,
        }
    }

    pub fn to_id<S>(contest_choice: &ContestChoice, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(contest_choice.id)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ContestResult {
    pub contest: Contest,
    pub total_valid_votes: i64,
    // For this exercise a vote is invalid if:
    // DecodedContestVote::is_explicit_invalid is set to true, or
    // The number of selected choices does not comply with Contest::min/max_choices
    pub total_invalid_votes: i64,
    // The counts per choice
    pub results: Vec<ContestChoiceResult>,
    // The winners for the contest (see Contest:num_winners)
    pub winners: Vec<ContestChoice>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ContestChoiceResult {
    // pub contest_result: ContestResult, // commented because this causes infinite loop
    pub contest_choice: ContestChoice,
    pub total_count: u64,
    // If a winner, the position of this choice (eg 1st, 2nd), otherwise 0
    // Ties are handled by using duplicates, eg 1st, 1st, 3rd..
    pub winner_position: u64,
}
