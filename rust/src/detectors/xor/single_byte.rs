use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::crackers::xor::single_byte::crack;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct DetectEntryResult {
    pub score: usize,
    pub key: u8,
    pub plain_text: Vec<u8>,
    pub index: usize,
}

pub fn detect_in_list<L, E>(entry_list: L) -> DetectEntryResult
where
    L: AsRef<[E]>,
    E: AsRef<[u8]>,
{
    let entry_list = entry_list.as_ref();
    let mut output = DetectEntryResult {
        score: 0,
        key: 0,
        plain_text: vec![],
        index: 0,
    };

    for (i, entry) in entry_list.iter().enumerate() {
        let result = crack(entry);
        if result.score > output.score {
            output.score = result.score;
            output.key = result.key;
            output.plain_text = result.plain_text;
            output.index = i;
        }
    }
    output
}

pub fn detect_in_file<P>(path: P) -> Result<DetectEntryResult>
where
    P: AsRef<Path>,
{
    let file = BufReader::new(File::open(path)?);

    let mut data = vec![];
    for line in file.lines() {
        data.push(hex::decode(line?)?);
    }
    Ok(detect_in_list(data))
}
