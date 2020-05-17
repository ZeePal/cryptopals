extern crate cryptopals;

use super::super::utils::get_resources_folder;

use cryptopals::detectors::aes::ecb::detect_in_file;

// Source: https://cryptopals.com/sets/1/challenges/8
#[test]
fn test() {
    let mut input_file = get_resources_folder(module_path!());
    input_file.push("input.txt");
    let expected_line_number = 133;

    let result = detect_in_file(input_file).unwrap();

    assert_eq!(result.index + 1, expected_line_number);
}
