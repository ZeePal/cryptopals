extern crate cryptopals;

use super::super::utils::get_resources_folder;

use cryptopals::ciphers::aes::ecb::decrypt_file;

// Source: https://cryptopals.com/sets/1/challenges/7
#[test]
fn test() {
    let mut input_file = get_resources_folder(module_path!());
    input_file.push("input.txt");
    let input_key = "YELLOW SUBMARINE";

    let expected_starting = "I'm back and I'm ringin' the bell";

    let result = decrypt_file(input_file, input_key).unwrap();

    assert_eq!(
        &result[0..expected_starting.len()],
        expected_starting.as_bytes()
    );
}
