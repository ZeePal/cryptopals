extern crate cryptopals;

use super::super::utils::get_resources_folder;

use cryptopals::ciphers::aes::cbc::decrypt_file;

// Source: https://cryptopals.com/sets/2/challenges/10
#[test]
fn test() {
    let mut input_file = get_resources_folder(module_path!());
    input_file.push("input.txt");
    let input_key = "YELLOW SUBMARINE";
    let input_iv = vec![0; 16];

    let expected_starting = "I'm back and I'm ringin' the bell";

    let result = decrypt_file(input_file, input_key, input_iv).unwrap();

    assert_eq!(
        &result[0..expected_starting.len()],
        expected_starting.as_bytes()
    );
}
