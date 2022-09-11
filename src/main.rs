use std::env;
use std::path::PathBuf;
use std::process;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: loxrs [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let file: PathBuf = PathBuf::from(&args[1]);
        loxrs::run_file(file)
    } else {
        loxrs::run_prompt();
        Ok(())
    }
}
