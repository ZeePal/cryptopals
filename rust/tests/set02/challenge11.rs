extern crate cryptopals;

use rand::prelude::*;

use cryptopals::detectors::aes::ecb::detect;
use cryptopals::oracles::aes::random::{aes_cbc, aes_ecb};

// Source: https://cryptopals.com/sets/2/challenges/11
#[test]
fn test() {
    let input = vec![0; 64];

    assert_eq!(detect(aes_cbc(&input)), 0);
    assert!(detect(aes_ecb(&input)) > 0)
}

#[test]
fn random() {
    let mut rng = rand::thread_rng();
    let input = vec![0; 64];

    for _ in 0..50 {
        match rng.gen() {
            true => assert_eq!(detect(aes_cbc(&input)), 0),
            false => assert!(detect(aes_ecb(&input)) > 0),
        }
    }
}
