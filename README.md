# chewy

**A silly little cli utility I made to learn some Rust**

### install

```sh
# clone the repo
➜ git clone https://github.com/thewemonster/chewy
# go into the directory
➜ cd chewy
# build the program
➜ cargo build
```
### basic usage

```sh
Silly little rust program that splits email addresses into three parts, username, delimeter, and domain.

Usage: chewy [OPTIONS]

Options:
  -i, --infile <INFILE>    (Optional) Path to file from which to read email addresses, otherwise pipe input
  -o, --outfile <OUTFILE>  (Optional) Path to output file, otherwise output will be stdout
  -u, --username           (Optional) Limits output to usernames
  -d, --domain             (Optional) Limits output to domains
  -h, --help               Print help information
  -V, --version            Print version information
```

### future development

Looking forward to continue learning Rust. I'd like to implement some more elegant error handling and find ways to be more efficient. If you have suggestions, let me know!
