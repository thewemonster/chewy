#![allow(unused)]

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    /// the input from the user; supposed to be an email.
    input: String,
}

struct Email {
    // for example [username: myuname, extra_qualifier: +, domain: gmail.com]
    username: String,
    extra_qualifier: String,
    domain: String,
}

impl Email {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            extra_qualifier: String::new(),
            domain: String::new(),
        }
    }
    fn to_vec(&self) -> Vec<&str> {
        vec![&self.username, &self.extra_qualifier, &self.domain]
    }
}

fn input_separation(mut email: Email, input: String) -> Email {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if email_regex.is_match(&input) {
        let v: Vec<&str> = input.split(&['+', '@'][..]).collect();
        if v.len() == 3 {
            email.username = String::from(v[0]);
            email.extra_qualifier = String::from(v[1]);
            email.domain = String::from(v[2]);
        } else if v.len() == 2 {
            email.username = String::from(v[0]);
            email.domain = String::from(v[1]);
        } else {
            panic!("Invalid email");
        }
    } else {
        panic!("Invalid email");
    }
    return email;
}

fn print_table_header() {
    println!(
        "{0: <15} | {1: <15} | {2: <15}",
        "Username", "Extra Qualifier", "Domain"
    );
}

fn prettyprint_email(email: Email) {
    let v: Vec<&str> = email.to_vec();
    println!("{0: <15} | {1: <15} | {2: <15}", v[0], v[1], v[2]);
}

fn main() {
    let args = Cli::parse();
    let mut email = input_separation(Email::new(), args.input);
    print_table_header();
    prettyprint_email(email);
}
