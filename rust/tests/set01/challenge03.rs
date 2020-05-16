extern crate cryptopals;
extern crate hex;

use std::str::from_utf8;

use cryptopals::crackers::xor::single_byte::crack;

// Source: https://cryptopals.com/sets/1/challenges/3
#[test]
fn test() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let expected_plain_text = "Cooking MC's like a pound of bacon";
    let expected_key = 'X' as u8;

    let decoded_input = hex::decode(input).unwrap();

    let result = crack(decoded_input);
    let plain_text = from_utf8(&result.plain_text).unwrap();

    assert_eq!(plain_text, expected_plain_text);
    assert_eq!(result.key, expected_key);
}
