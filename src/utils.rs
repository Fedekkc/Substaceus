use std::fs::File;
use std::io::{BufReader, Result};

pub fn open_wordlist(path: &str) -> Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
