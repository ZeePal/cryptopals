use std::error::Error;

use hex::decode as hex_decode;
use hex::encode as hex_encode;

pub fn xor<T: AsRef<[u8]>, X: AsRef<[u8]>>(left: T, right: X) -> Result<Vec<u8>, &'static str> {
    let left = left.as_ref();
    let right = right.as_ref();

    let length = left.len();
    if length != right.len() {
        return Err("Inputs aren't of equal length");
    }

    let mut output = vec![];
    for i in 0..length {
        output.push(left[i] ^ right[i]);
    }

    Ok(output)
}

pub fn xor_hex<T: AsRef<[u8]>, X: AsRef<[u8]>>(
    left: T,
    right: X,
) -> Result<String, Box<dyn Error>> {
    let left = hex_decode(left.as_ref())?;
    let right = hex_decode(right.as_ref())?;

    Ok(hex_encode(xor(left, right)?))
}
