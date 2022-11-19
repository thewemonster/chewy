# chewy

**A silly little utility I made to learn some Rust**

### install & run

```sh
# clone the repo
➜ git clone https://github.com/thewemonster/chewy
# go into the directory
➜ cd chewy
# build the program
➜ cargo build
# run the program with the expected argument
➜ cargo run -- "asdf+123@asdf.com"
```
### basic usage

```sh
# the program takes a single email (for now) and spits out three parts of the
# email: username, filtering delimeter, and the domain. Example inputs include:
# username@domain.com, username+qualifier@domain.com,
# uname.things@sub.domain.com, uname.things+qual@sub.domain.com
➜ cargo run -- "asdf+123@asdf.com"
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/chewy 'asdf+123@asdf.com'`
Username        | Extra Qualifier | Domain
asdf            | 123             | asdf.com
```

### future development

Looking forward to continue learning Rust. I'd like to implement some better
error handling, accept a file with email addresses, and find ways to be more efficient. If you have suggestions, let me know!
