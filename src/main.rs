/*
Author: thewemonster
Email: wes@thewemonster.com
Website: https://thewemonster.com
Purpose: Learn rust by implementing some basics of the language
*/

#![allow(unused)]

use clap::{command, ArgAction, ArgGroup, Parser};
use regex::Regex;
use std::{
    fs::{self, File},
    io::{self, prelude::*, stdin, stdout, BufReader, BufWriter},
    path::PathBuf,
    process,
    ptr::read,
};

#[derive(Parser)]
#[command(version, about, author, group(ArgGroup::new("DataOutput").args(["username", "domain"])))]
struct Cli {
    /// (Optional) Path to file from which to read email addresses, otherwise pipe input
    #[arg(short, long)]
    infile: Option<PathBuf>,
    /// (Optional) Path to output file, otherwise output will be stdout
    #[arg(short, long)]
    outfile: Option<PathBuf>,
    /// (Optional) Limits output to usernames
    #[arg(short, long, group = "DataOutput")]
    username: bool,
    /// (Optional) Limits output to domains
    #[arg(short, long, group = "DataOutput")]
    domain: bool,
}

/// constant to specify an email regular expression
const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

enum DataOutput {
    Username,
    Domain,
    Everything,
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
                _ => fail_gracefully("invalid email."),
            };
        }
        fail_gracefully("regex unable to match an email.");
    }

    fn prettyprint_to_string(&self) -> String {
        format!(
            "{0: <15} | {1: <15} | {2: <15}",
            &self.username, &self.delimeter, &self.domain
        )
    }
}

fn triage_output (
    mut reader: BufReader<Box<dyn Read>>,
    mut writer: BufWriter<Box<dyn Write>>,
    data_out_style: DataOutput,
) {
    match data_out_style {
        DataOutput::Username => {
            for line in reader.lines() {
                writer.write(Email::from(line.unwrap()).username.as_bytes());
                writer.write(b"\n");
            }
        }
        DataOutput::Domain => {
            for line in reader.lines() {
                writer.write(Email::from(line.unwrap()).domain.as_bytes());
                writer.write(b"\n");
            }
        }
        DataOutput::Everything => {
            prettyprint_table_headers(&mut writer);
            for line in reader.lines() {
                writer.write(
                    Email::from(line.unwrap())
                        .prettyprint_to_string()
                        .as_bytes(),
                );
                writer.write(b"\n");
            }
        }
    }
}

fn prettyprint_table_headers(writer: &mut BufWriter<Box<dyn Write>>) -> Result<usize, io::Error> {
    writer.write(
        format!(
            "{0: <15} | {1: <15} | {2: <15}",
            "Username", "Delimeter", "Domain"
        )
        .as_bytes(),
    );
    writer.write(b"\n")
}

fn fail_gracefully(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1)
}

/// main
fn main() -> io::Result<()> {
    let args = Cli::parse();

    let data_out_style = {
        if args.domain {
            DataOutput::Domain
        } else if args.username {
            DataOutput::Username
        } else {
            DataOutput::Everything
        }
    };

    let mut reader: BufReader<Box<dyn Read>> = match args.infile {
        Some(read_path) => BufReader::new(Box::new(File::open(read_path)?)),
        None => BufReader::new(Box::new(stdin().lock())),
    };

    let mut writer: BufWriter<Box<dyn Write>> = match args.outfile {
        Some(writer_path) => BufWriter::new(Box::new(File::create(writer_path)?)),
        None => BufWriter::new(Box::new(stdout().lock())),
    };

    triage_output(reader, writer, data_out_style);

    Ok(())
}
