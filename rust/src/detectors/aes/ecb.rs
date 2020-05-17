use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub struct DetectEntryResult {
    pub score: usize,
    pub index: usize,
}

pub fn detect<T: AsRef<[u8]>>(data: T) -> usize {
    let data = data.as_ref();

    let mut found = HashSet::new();
    let mut score = 0;
    for chunk in data.chunks_exact(16) {
        if !found.insert(chunk) {
            score += 1;
        }
    }

    score
}

pub fn detect_in_possibles(data: &Vec<Vec<u8>>) -> DetectEntryResult {
    let mut output = DetectEntryResult { score: 0, index: 0 };

    for (i, entry) in data.iter().enumerate() {
        let score = detect(&entry);
        if score > output.score {
            output.score = score;
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
