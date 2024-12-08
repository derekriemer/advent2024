use advent2024::{day01};
use std::env;
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <day> ", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];

    let day_out: Result<String> = match day.as_str() {
        "1.1" => day01::run_puzzle_1(),
        d => Err(anyhow!("Invalid day: {}",d)),
    };
    println!("{}", day_out.unwrap_or_else(|err| err.to_string())); // To print or handle error
    Ok(())
}
