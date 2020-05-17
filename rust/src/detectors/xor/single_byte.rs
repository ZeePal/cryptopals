use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::crackers::xor::single_byte::crack;

pub struct DetectEntryResult {
    pub score: usize,
    pub key: u8,
    pub plain_text: Vec<u8>,
    pub index: usize,
}

pub fn detect_in_possibles(data: &Vec<Vec<u8>>) -> DetectEntryResult {
    let mut output = DetectEntryResult {
        score: 0,
        key: 0,
        plain_text: vec![],
        index: 0,
    };

    for (i, entry) in data.iter().enumerate() {
        let result = crack(&entry);
        if result.score > output.score {
            output.score = result.score;
            output.key = result.key;
            output.plain_text = result.plain_text;
            output.index = i;
        }
    }

    output
}

pub fn detect_in_file<P: AsRef<Path>>(path: P) -> Result<DetectEntryResult, Box<dyn Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut data = vec![];
    for line in file.lines() {
        data.push(hex::decode(line?)?);
    }
    Ok(detect_in_possibles(&data))
}
