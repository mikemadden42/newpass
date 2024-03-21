use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{env, process};

fn main() {
    for argument in env::args().skip(1) {
        // Attempt to parse the argument into an integer
        let length: usize = match argument.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error: Couldn't parse '{argument}' as a valid number");
                process::exit(1);
            }
        };

        // Generate random string of given length
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        println!("{rand_string}");
    }
}
