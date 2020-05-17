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

pub fn detect(data: &Vec<Vec<u8>>) -> DetectEntryResult {
    let mut output = DetectEntryResult { score: 0, index: 0 };

    for (i, entry) in data.iter().enumerate() {
        let mut found = HashSet::new();
        let mut score = 0;
        for chunk in entry.chunks_exact(16) {
            if !found.insert(chunk) {
                score += 1;
            }
        }
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
    Ok(detect(&data))
}
