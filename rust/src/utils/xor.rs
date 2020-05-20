use hex::decode as hex_decode;
use hex::encode as hex_encode;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn xor<L, R>(mut left: L, right: R)
where
    L: AsMut<[u8]>,
    R: AsRef<[u8]>,
{
    let left = left.as_mut();
    let right = right.as_ref();

    for (a, b) in left.iter_mut().zip(right.iter()) {
        *a ^= b
    }
}

pub fn xor_hex<L, R>(left: L, right: R) -> Result<String>
where
    L: AsRef<[u8]>,
    R: AsRef<[u8]>,
{
    let mut left = hex_decode(left.as_ref())?;
    let right = hex_decode(right.as_ref())?;

    xor(&mut left, right);
    Ok(hex_encode(left))
}
