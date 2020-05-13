use crate::ciphers::xor::single_byte::crypt;
use crate::text_scoring::common::count as get_score;

pub fn crack<T: AsRef<[u8]>>(raw_cipher_text: T) -> (Vec<u8>, u8, usize) {
    let mut best_score = 0;
    let mut best_key = 0;
    let mut best_plain_text = vec![];

    for i in 0..=255 {
        let plain_text = crypt(i, &raw_cipher_text);
        let score = get_score(&plain_text);
        if score > best_score {
            best_score = score;
            best_plain_text = plain_text;
            best_key = i;
        }
    }

    (best_plain_text, best_key, best_score)
}
