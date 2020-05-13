extern crate cryptopals;

use std::str::from_utf8;

use super::super::utils::get_resources_folder;

use cryptopals::detectors::xor::single_byte::detect;

// Source: https://cryptopals.com/sets/1/challenges/4
#[test]
fn test() {
    let mut input_file = get_resources_folder(module_path!());
    input_file.push("input.txt");
    let expected_plain_text = "Now that the party is jumping\n";
    let expected_key = '5' as u8;
    let expected_line_number = 171;

    let (plain_text_in_bytes, key, _, line_number) = detect(input_file).unwrap();

    let plain_text = from_utf8(&plain_text_in_bytes).unwrap();

    assert_eq!(plain_text, expected_plain_text);
    assert_eq!(key, expected_key);
    assert_eq!(line_number, expected_line_number);
}
