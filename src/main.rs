extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;

fn main() {
    for argument in env::args().skip(1) {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(argument.parse().unwrap())
            .map(char::from)
            .collect();
        println!("{}", rand_string);
    }
}
