use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct DetectEntryResult {
    pub score: usize,
    pub index: usize,
}

pub fn detect<C>(cipher_text: C) -> usize
where
    C: AsRef<[u8]>,
{
    let cipher_text = cipher_text.as_ref();

    let mut found = HashSet::new();
    let mut score = 0;
    for chunk in cipher_text.chunks_exact(16) {
        if !found.insert(chunk) {
            score += 1; // Failed to insert a chunk as it already exists in the HashSet
        }
    }
    score
}

pub fn detect_in_list<L, E>(entry_list: L) -> DetectEntryResult
where
    L: AsRef<[E]>,
    E: AsRef<[u8]>,
{
    let entry_list = entry_list.as_ref();
    let mut output = DetectEntryResult { score: 0, index: 0 };

    for (i, entry) in entry_list.iter().enumerate() {
        let score = detect(entry);
        if score > output.score {
            output.score = score;
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
