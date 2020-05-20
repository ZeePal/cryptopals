use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use base64::decode as b64_decode;

use crate::crackers::xor::single_byte::crack as single_byte_cracker;
use crate::detectors::xor::repeating_bytes::detect_keysizes;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct CrackResult {
    pub score: usize,
    pub key: Vec<u8>,
}

// Example arguments: Vec<u8>, 2, 40, 5, 3
pub fn crack<C>(
    cipher_text: C,
    min_key_size: usize,
    max_key_size: usize,
    samples: usize,
    check_top_x_key_sizes: usize,
) -> CrackResult
where
    C: AsRef<[u8]>,
{
    let cipher_text = cipher_text.as_ref();
    let mut output = CrackResult {
        score: 0,
        key: vec![],
    };

    let keysizes = detect_keysizes(
        cipher_text,
        min_key_size,
        max_key_size,
        samples,
        check_top_x_key_sizes,
    );

    for keysize in keysizes {
        let cipher_text_transposed = transpose(cipher_text, keysize.size);

        let mut score = 0;
        let mut key = Vec::with_capacity(keysize.size);
        for i in 0..keysize.size {
            let result = single_byte_cracker(&cipher_text_transposed[i]);
            key.push(result.key);
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
pub fn crack_file<P>(
    path: P,
    min_key_size: usize,
    max_key_size: usize,
    samples: usize,
    check_top_x_key_sizes: usize,
) -> Result<CrackResult>
where
    P: AsRef<Path>,
{
    let file = BufReader::new(File::open(path)?);

    let data = b64_decode(
        file.bytes()
            .map(|x| x.unwrap())
            .filter(|&x| x != b'\n')
            .collect::<Vec<u8>>(),
    )?;
    Ok(crack(
        data,
        min_key_size,
        max_key_size,
        samples,
        check_top_x_key_sizes,
    ))
}

fn transpose(cipher_text: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let inner_capacity = cipher_text.len() / keysize;
    let mut output = vec![Vec::with_capacity(inner_capacity); keysize];

    for chunk in cipher_text.chunks_exact(keysize) {
        for i in 0..keysize {
            output[i].push(chunk[i]);
        }
    }
    output
}
