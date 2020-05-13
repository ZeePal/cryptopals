use base64::encode as b64_encode;
use hex::decode as hex_decode;
use hex::FromHexError;

pub fn hex_to_base64<T: AsRef<[u8]>>(input: T) -> Result<String, FromHexError> {
    Ok(b64_encode(hex_decode(input)?))
}
