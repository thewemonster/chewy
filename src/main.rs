/*
Author: thewemonster
Purpose: Learn rust by implementing some basics of the language
*/

#![allow(unused)]

use clap::Parser;
use regex::Regex;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

#[derive(Parser)]
struct Cli {
    // the input from the user; supposed to be an email.
    // accepts things like username@domain.com, username+qualifier@domain.com,
    // uname.things@sub.domain.com. uname.things+qual@sub.domain.com
    input: String,
}

#[derive(Debug)]
struct Email {
    username: String,
    extra_qualifier: String,
    domain: String,
}

impl Email {
    pub fn new(input: String) -> Self {
        if Regex::new(&EMAIL_REGEX).unwrap().is_match(&input) {
            let items: Vec<&str> = input.split(&['+', '@'][..]).collect();
            return match items.len() {
                2 => Self {
                    username: String::from(items[0]),
                    extra_qualifier: String::new(),
                    domain: String::from(items[1]),
                },
                3 => Self {
                    username: String::from(items[0]),
                    extra_qualifier: String::from(items[1]),
                    domain: String::from(items[2]),
                },
                _ => panic!("invalid email components."),
            };
        }
        panic!("invalid email.");
    }
    fn prettyprint(&self) {
        println!(
            "{0: <15} | {1: <15} | {2: <15}",
            &self.username, &self.extra_qualifier, &self.domain
        );
    }
}

fn print_table_header() {
    println!(
        "{0: <15} | {1: <15} | {2: <15}",
        "Username", "Extra Qualifier", "Domain"
    );
}

fn main() {
    let args = Cli::parse();
    let email = Email::new(args.input);
    print_table_header();
    email.prettyprint();
}
