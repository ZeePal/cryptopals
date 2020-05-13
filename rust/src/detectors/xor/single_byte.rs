use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::crackers::xor::single_byte::crack;

pub fn detect<P: AsRef<Path>>(path: P) -> Result<(Vec<u8>, u8, usize, usize), Box<dyn Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut best_score = 0;
    let mut best_plain_text = vec![];
    let mut best_key = 0;
    let mut best_line_number = 0;

    for (i, line) in file.lines().enumerate() {
        let (plain_text, key, score) = crack(hex::decode(line?)?);
        if score > best_score {
            best_score = score;
            best_plain_text = plain_text;
            best_key = key;
            best_line_number = i + 1;
        }
    }

    Ok((best_plain_text, best_key, best_score, best_line_number))
}
