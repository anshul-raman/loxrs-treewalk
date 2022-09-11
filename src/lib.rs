use anyhow::Result;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub fn run(input: String) -> Result<()> {
    dbg!(input);
    Ok(())
}

pub fn run_file(file: PathBuf) -> Result<()> {
    dbg!(file);
    Ok(())
}

pub fn run_prompt() -> io::Result<()> {
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
