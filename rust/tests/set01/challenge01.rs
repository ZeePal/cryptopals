extern crate cryptopals;

// Source: https://cryptopals.com/sets/1/challenges/1
#[test]
fn test() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    let result = cryptopals::utils::hex_to_base64(input).unwrap();

    assert_eq!(result, expected);
}
