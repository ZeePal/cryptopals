use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use base64::decode as b64_decode;

use crate::crackers::xor::single_byte::crack as single_byte_cracker;
use crate::detectors::xor::repeating_bytes::detect_keysizes;

// Example arguments: "/tmp/input.txt", 2, 40, 5, 3
pub fn crack<P: AsRef<Path>>(
    path: P,
    min_key_size: usize,
    max_key_size: usize,
    samples: usize,
    check_top_x_key_sizes: usize,
) -> Result<(Vec<u8>, usize), Box<dyn Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let data = b64_decode(
        file.bytes()
            .map(|x| x.unwrap())
            .filter(|&x| x != 10)
            .collect::<Vec<u8>>(),
    )?;

    let possible_keysizes = detect_keysizes(
        &data,
        min_key_size,
        max_key_size,
        samples,
        check_top_x_key_sizes,
    );

    let mut best_score = 0;
    let mut best_key = vec![];

    for keysize in possible_keysizes {
        let data_transposed = transpose(&data, keysize.size);

        let mut score = 0;
        let mut key = vec![];
        for i in 0..keysize.size {
            let (_, char, char_score) = single_byte_cracker(&data_transposed[i]);
            key.push(char);
            score += char_score;
        }

        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }

    Ok((best_key, best_score))
}

fn transpose<T: AsRef<[u8]>>(data: T, keysize: usize) -> Vec<Vec<u8>> {
    let data = data.as_ref();
    let inner_capacity = data.len() / keysize;
    let mut output = vec![Vec::with_capacity(inner_capacity); keysize];

    for chunk in data.chunks_exact(keysize) {
        for i in 0..keysize {
            output[i].push(chunk[i]);
        }
    }

    output
}
