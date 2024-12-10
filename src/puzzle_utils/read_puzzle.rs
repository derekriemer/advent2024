use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

/// Reads puzzle input from a file and returns it as a String.
pub fn read_puzzle(file_path: &str) -> Result<String> {
    let mut file =
        File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| "Could not read from file {file_path}")?;
    Ok(contents)
}
