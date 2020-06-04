use crate::ciphers::caesar::*;
use crate::text_scoring::common::count as get_score;

#[derive(Debug)]
pub struct CrackResult {
    pub score: usize,
    pub key: u8,
    pub shift: Shift,
    pub plain_text: Vec<u8>,
}

pub fn crack<C>(cipher_text: C) -> CrackResult
where
    C: AsRef<[u8]>,
{
    let cipher_text = cipher_text.as_ref();
    let mut output = CrackResult {
        score: 0,
        key: 0,
        shift: Shift::Right,
        plain_text: vec![],
    };

    for i in 0..=ALPHABET_SIZE {
        let mut plain_text = cipher_text.to_vec();
        crypt(&mut plain_text, i, Shift::Right);

        let score = get_score(&plain_text);
        if score > output.score {
            output.score = score;
            output.plain_text = plain_text;
            output.key = i;
        }
    }
    output
}
