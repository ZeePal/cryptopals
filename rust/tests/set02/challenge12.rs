extern crate cryptopals;

use cryptopals::crackers::aes::oracle_prefixing::crack;
use cryptopals::oracles::aes::ecb::prefixing::Oracle;

// Source: https://cryptopals.com/sets/2/challenges/12
#[test]
fn test() {
    let input = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK"
        .replace("\n", "");

    let input_decoded = base64::decode(input).unwrap();
    let oracle = Oracle::new(&input_decoded);

    let result = crack(&oracle).unwrap();
    assert_eq!(result, input_decoded);
}
