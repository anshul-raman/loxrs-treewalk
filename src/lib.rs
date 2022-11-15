mod errors;
mod scanner;
mod token;

use anyhow::Result;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use scanner::Scanner;

pub fn run(input: String) -> Result<()> {
    let mut scanner = Scanner::new(&input);

    let tokens = scanner.scan_tokens();
    dbg!("{}", tokens);

    Ok(())
}

pub fn run_file(file: PathBuf) -> Result<()> {
    let source_code = fs::read_to_string(file).expect("Unable to read input file");
    run(source_code)
}

pub fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        run(line?);
        print!("> ");
        io::stdout().flush().unwrap();
    }
    Ok(())
}
