use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use std::{env, process};

fn main() {
    for argument in env::args().skip(1) {
        // Attempt to parse the argument into an integer
        let length: usize = if let Ok(num) = argument.parse() {
            num
        } else {
            eprintln!("Error: Could not parse '{argument}' as a valid number");
            process::exit(1);
        };

        // Generate random string of given length
        let rand_string: String = rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        println!("{rand_string}");
    }
}
