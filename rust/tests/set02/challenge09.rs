extern crate cryptopals;

use cryptopals::utils::pad;

// Source: https://cryptopals.com/sets/2/challenges/9
#[test]
fn test() {
    let input = "YELLOW SUBMARINE".as_bytes().to_vec();
    let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes().to_vec();

    let mut result = input.clone();
    pad(&mut result, 20);

    assert_eq!(result, expected);
}
