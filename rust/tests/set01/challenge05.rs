extern crate cryptopals;

use cryptopals::ciphers::xor::repeating_bytes::crypt;

// Source: https://cryptopals.com/sets/1/challenges/5
#[test]
fn test() {
    let input_data = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let input_key = "ICE";
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        .replace("\n", "");

    let result = crypt(input_key, input_data);
    let result_hex = hex::encode(result);

    assert_eq!(result_hex, expected);
}
