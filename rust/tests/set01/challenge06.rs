extern crate cryptopals;

use super::super::utils::get_resources_folder;

use cryptopals::crackers::xor::repeating_bytes::crack_file;

// Source: https://cryptopals.com/sets/1/challenges/6
#[test]
fn test() {
    let mut input_file = get_resources_folder(module_path!());
    input_file.push("input.txt");
    let expected_key = "Terminator X: Bring the noise".as_bytes();

    let result = crack_file(input_file, 2, 40, 5, 3).unwrap();

    assert_eq!(result.key, expected_key);
}
