use rand::{rng, Rng};
use std::{env, process};

const DEFAULT_LENGTH: usize = 16;
const MAX_LENGTH: usize = 1024;

const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
// Quotes, backslash and backtick are left out so passwords are easy to paste into a shell
const SYMBOLS: &[u8] = b"!#$%&()*+,-./:;<=>?@[]^_{|}~";

const USAGE: &str = "\
Usage: newpass [OPTIONS] [LENGTH]...

Generate one random password per LENGTH (default: 16, max: 1024).

Options:
  -s, --symbols  Include symbols as well as letters and digits
  -h, --help     Print this help";

#[derive(Debug, PartialEq)]
enum Command {
    Help,
    Generate { symbols: bool, lengths: Vec<usize> },
}

fn parse_length(arg: &str) -> Result<usize, String> {
    match arg.parse::<usize>() {
        Ok(n) if (1..=MAX_LENGTH).contains(&n) => Ok(n),
        Ok(_) => Err(format!(
            "length must be between 1 and {MAX_LENGTH}, got '{arg}'"
        )),
        Err(_) => Err(format!("could not parse '{arg}' as a valid number")),
    }
}

// Validate every argument up front so a bad one doesn't leave partial output
fn parse_args<I: IntoIterator<Item = String>>(args: I) -> Result<Command, String> {
    let mut symbols = false;
    let mut lengths = Vec::new();

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-s" | "--symbols" => symbols = true,
            _ if arg.starts_with('-') => return Err(format!("unknown option '{arg}'")),
            _ => lengths.push(parse_length(&arg)?),
        }
    }

    if lengths.is_empty() {
        lengths.push(DEFAULT_LENGTH);
    }

    Ok(Command::Generate { symbols, lengths })
}

fn generate(length: usize, charset: &[u8]) -> String {
    let mut rng = rng();
    (0..length)
        .map(|_| char::from(charset[rng.random_range(0..charset.len())]))
        .collect()
}

fn main() {
    let (symbols, lengths) = match parse_args(env::args().skip(1)) {
        Ok(Command::Help) => {
            println!("{USAGE}");
            return;
        }
        Ok(Command::Generate { symbols, lengths }) => (symbols, lengths),
        Err(message) => {
            eprintln!("Error: {message}\n\n{USAGE}");
            process::exit(1);
        }
    };

    let charset = if symbols {
        [ALPHANUMERIC, SYMBOLS].concat()
    } else {
        ALPHANUMERIC.to_vec()
    };

    for length in lengths {
        println!("{}", generate(length, &charset));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn defaults_to_one_password() {
        assert_eq!(
            parse_args(args(&[])),
            Ok(Command::Generate {
                symbols: false,
                lengths: vec![DEFAULT_LENGTH]
            })
        );
    }

    #[test]
    fn parses_lengths_and_symbols_flag() {
        assert_eq!(
            parse_args(args(&["8", "--symbols", "32"])),
            Ok(Command::Generate {
                symbols: true,
                lengths: vec![8, 32]
            })
        );
    }

    #[test]
    fn help_flag() {
        assert_eq!(parse_args(args(&["12", "-h"])), Ok(Command::Help));
    }

    #[test]
    fn rejects_out_of_range_lengths() {
        assert!(parse_length("0").is_err());
        assert!(parse_length(&(MAX_LENGTH + 1).to_string()).is_err());
        assert_eq!(parse_length(&MAX_LENGTH.to_string()), Ok(MAX_LENGTH));
    }

    #[test]
    fn rejects_invalid_arguments() {
        assert!(parse_args(args(&["16", "abc"])).is_err());
        assert!(parse_args(args(&["-5"])).is_err());
        assert!(parse_args(args(&["--bogus"])).is_err());
    }

    #[test]
    fn generates_requested_length_from_charset() {
        let password = generate(200, ALPHANUMERIC);
        assert_eq!(password.len(), 200);
        assert!(password.bytes().all(|b| ALPHANUMERIC.contains(&b)));
    }
}
