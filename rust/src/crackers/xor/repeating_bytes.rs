use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use base64::decode as b64_decode;

use crate::crackers::xor::single_byte::crack as single_byte_cracker;
use crate::detectors::xor::repeating_bytes::detect_keysizes;

pub struct CrackResult {
    pub score: usize,
    pub key: Vec<u8>,
}

// Example arguments: Vec<u8>, 2, 40, 5, 3
pub fn crack<T: AsRef<[u8]>>(
    data: T,
    min_key_size: usize,
    max_key_size: usize,
    samples: usize,
    check_top_x_key_sizes: usize,
) -> CrackResult {
    let data = data.as_ref();
    let mut output = CrackResult {
        score: 0,
        key: vec![],
    };

    let keysizes = detect_keysizes(
        data,
        min_key_size,
        max_key_size,
        samples,
        check_top_x_key_sizes,
    );

    for keysize in keysizes {
        let data_transposed = transpose(data, keysize.size);

        let mut score = 0;
        let mut key = vec![0; keysize.size];
        for i in 0..keysize.size {
            let result = single_byte_cracker(&data_transposed[i]);
            key[i] = result.key;
            score += result.score;
        }

        if score > output.score {
            output.score = score;
            output.key = key;
        }
    }

    output
}

// Example arguments: "/tmp/input.txt", 2, 40, 5, 3
pub fn crack_file<P: AsRef<Path>>(
    path: P,
    min_key_size: usize,
    max_key_size: usize,
    samples: usize,
    check_top_x_key_sizes: usize,
) -> Result<CrackResult, Box<dyn Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let data = b64_decode(
        file.bytes()
            .map(|x| x.unwrap())
            .filter(|&x| x != 10) // Ignore newline characters
            .collect::<Vec<u8>>(),
    )?;
    Ok(crack(
        &data,
        min_key_size,
        max_key_size,
        samples,
        check_top_x_key_sizes,
    ))
}

fn transpose<T: AsRef<[u8]>>(data: T, keysize: usize) -> Vec<Vec<u8>> {
    let data = data.as_ref();

    let mut output = vec![Vec::with_capacity(data.len() / keysize); keysize];
    for chunk in data.chunks_exact(keysize) {
        for i in 0..keysize {
            output[i].push(chunk[i]);
        }
    }

    output
}
