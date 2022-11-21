/*
Author: thewemonster
Purpose: Learn rust by implementing some basics of the language
*/

#![allow(unused)]

use clap::{command, ArgAction, Parser};
use regex::Regex;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
    process,
};

/// constant to specify an email regular expression
const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    // the input from the user; supposed to be an email.
    // accepts things like username@domain.com, username+qualifier@domain.com,
    // uname.things@sub.domain.com. uname.things+qual@sub.domain.com
    /// (Optional) Path to file that contains email addresses.
    #[arg(short, long)]
    path: Option<PathBuf>,
    /// (Optional) Flag indicating output should only render the usernames including the delimeter
    #[arg(short, long, action = ArgAction::SetTrue)]
    username_only: Option<bool>,
    /// (Optional) Flag indicating output should only render the domains
    #[arg(short, long, action = ArgAction::SetTrue)]
    domain_only: Option<bool>,
}

#[derive(Debug)]
/// Email struct definition
struct Email {
    username: String,
    delimeter: String,
    domain: String,
}

impl Email {
    /// creates a new Email and assigns default values: uname@domain.org
    pub fn new() -> Self {
        Self {
            username: String::from("uname"),
            delimeter: String::new(),
            domain: String::from("domain.org"),
        }
    }
    /// creates a new Email from the supplied String
    pub fn from(input: String) -> Self {
        if Regex::new(&EMAIL_REGEX).unwrap().is_match(&input) {
            let items: Vec<&str> = input.split(&['+', '@'][..]).collect();
            return match items.len() {
                2 => Self {
                    username: String::from(items[0]),
                    delimeter: String::new(),
                    domain: String::from(items[1]),
                },
                3 => Self {
                    username: String::from(items[0]),
                    delimeter: String::from(items[1]),
                    domain: String::from(items[2]),
                },
                _ => fail_gracefully("invalid number of email components."),
            };
        }
        fail_gracefully("regex unable to match an email.");
    }
    /// prettyprint's itself
    fn prettyprint(&self) {
        println!(
            "{0: <15} | {1: <15} | {2: <15}",
            &self.username, &self.delimeter, &self.domain
        );
    }
}

/// print's the table headers to the terminal
fn print_table_header() {
    println!(
        "{0: <15} | {1: <15} | {2: <15}",
        "Username", "Delimeter", "Domain"
    );
}

fn output(reader: BufReader<File>, uname: bool, domain: bool) {
    if uname {
        // print just each username
        for line in reader.lines() {
            println!("{}", Email::from(line.unwrap()).username);
        }
    } else if domain {
        // print just each domain
        for line in reader.lines() {
            println!("{}", Email::from(line.unwrap()).domain);
        }
    } else {
        // both false, print full
        print_table_header();
        for line in reader.lines() {
            Email::from(line.unwrap()).prettyprint();
        }
    }
}

fn fail_gracefully(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1)
}

/// main
fn main() -> io::Result<()> {
    let args = Cli::parse();
    let uname_only: bool = args.username_only.unwrap_or_else(|| (false));
    let domain_only: bool = args.domain_only.unwrap_or_else(|| (false));
    let reader = BufReader::new(File::open(args.path.unwrap())?);

    if uname_only && domain_only {
        fail_gracefully(
            "username-only and domain-only are mutually exclusive, please set only one.",
        );
    } else {
        output(reader, uname_only, domain_only);
    }

    Ok(())
}
