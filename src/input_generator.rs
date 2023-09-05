use crate::tally::Contest;
use fake::{Dummy, Fake, Faker};

pub fn generate_input() {
    let c = generate_contest();

    println!("{:?}", c);
}

fn generate_contest() -> Contest {
    Faker.fake()
}
