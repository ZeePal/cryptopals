use crate::ciphers::xor::single_byte::crypt;
use crate::text_scoring::common::count as get_score;

pub struct CrackResult {
    pub score: usize,
    pub key: u8,
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
        plain_text: vec![],
    };

    for i in 0..=255 {
        let mut plain_text = cipher_text.to_vec();
        crypt(&mut plain_text, i);

        let score = get_score(&plain_text);
        if score > output.score {
            output.score = score;
            output.key = i;
            output.plain_text = plain_text;
        }
    }
    output
}
