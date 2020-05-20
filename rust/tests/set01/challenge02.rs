extern crate cryptopals;
extern crate hex;

// Source: https://cryptopals.com/sets/1/challenges/2
#[test]
fn test() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";

    let result = cryptopals::utils::xor_hex(input1, input2).unwrap();

    assert_eq!(result, expected);
}
