use std::error::Error;

use hex::decode as hex_decode;
use hex::encode as hex_encode;

pub fn xor<T: AsRef<[u8]>>(left: &mut Vec<u8>, right: T) {
    let right = right.as_ref();

    for (a, b) in left.iter_mut().zip(right.iter()) {
        *a ^= b
    }
}

pub fn xor_hex<T: AsRef<[u8]>, X: AsRef<[u8]>>(
    left: T,
    right: X,
) -> Result<String, Box<dyn Error>> {
    let mut left = hex_decode(left.as_ref())?;
    let right = hex_decode(right.as_ref())?;

    xor(&mut left, right);
    Ok(hex_encode(left))
}
