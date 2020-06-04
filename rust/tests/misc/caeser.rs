extern crate cryptopals;

use cryptopals::ciphers::caesar::*;
use cryptopals::crackers::caesar::crack;

// Source: https://cryptohack.org/register/
#[test]
fn test1() {
    let input = b"GKSD WSXYB OKCSVI CMOXO";
    let expected = b"WAIT MINOR EASILY SCENE";

    assert_eq!(crack(input).plain_text, expected);
}

#[test]
fn test2() {
    let input = b"NOQBOO MBONSD DREWL DBSKV";
    let expected = b"DEGREE CREDIT THUMB TRIAL";

    assert_eq!(crack(input).plain_text, expected);
}

#[test]
fn test3() {
    let input = b"SLCO HCLA QTMPC ETDDFP";
    let expected = b"HARD WRAP FIBER TISSUE";

    assert_eq!(crack(input).plain_text, expected);
}

#[test]
fn cipher_right() {
    let mut input = vec![b'A', b'B', b'C', b'D'];
    let input_key = 1;
    let input_shift = Shift::Right;

    let expected = vec![b'B', b'C', b'D', b'E'];

    crypt(&mut input, input_key, input_shift);
    assert_eq!(input, expected);
}

#[test]
fn cipher_left() {
    let mut input = vec![b'B', b'C', b'D', b'E'];
    let input_key = 1;
    let input_shift = Shift::Left;

    let expected = vec![b'A', b'B', b'C', b'D'];

    crypt(&mut input, input_key, input_shift);
    assert_eq!(input, expected);
}
