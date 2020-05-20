use base64::encode as b64_encode;
use hex::decode as hex_decode;
use hex::FromHexError;

pub fn hex_to_base64<H>(hex: H) -> Result<String, FromHexError>
where
    H: AsRef<[u8]>,
{
    Ok(b64_encode(hex_decode(hex)?))
}
