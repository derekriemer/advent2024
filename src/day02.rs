use crate::puzzle_utils::read_puzzle::read_puzzle;
use anyhow::{Context, Result};

pub fn run_puzzle_1() -> Result<String> {
    let input = read_puzzle("puzzles/day2.txt")?;
    println!("{input}");
    Ok(String::from("5"))
}
