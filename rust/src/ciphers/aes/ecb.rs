use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use base64::decode as b64_decode;

use aesni::block_cipher_trait::generic_array::GenericArray;
use aesni::block_cipher_trait::BlockCipher;
use aesni::Aes128;

use crate::utils::pad;
use crate::utils::unpad;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const BLOCK_SIZE: u8 = 16;

pub fn decrypt_with_cipher(data: &mut Vec<u8>, cipher: Aes128) {
    for mut block in data.chunks_exact_mut(BLOCK_SIZE as usize) {
        cipher.decrypt_block(GenericArray::from_mut_slice(&mut block));
    }
    unpad(data, BLOCK_SIZE);
}

pub fn decrypt<K>(data: &mut Vec<u8>, key: K)
where
    K: AsRef<[u8]>,
{
    let key = key.as_ref();
    let cipher = Aes128::new(GenericArray::from_slice(key));
    decrypt_with_cipher(data, cipher);
}

pub fn decrypt_file<P, K>(path: P, key: K) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
    K: AsRef<[u8]>,
{
    let file = BufReader::new(File::open(path)?);

    let mut output = b64_decode(
        file.bytes()
            .map(|x| x.unwrap())
            .filter(|&x| x != b'\n')
            .collect::<Vec<u8>>(),
    )?;

    decrypt(&mut output, key);
    Ok(output)
}

pub fn encrypt_with_cipher(data: &mut Vec<u8>, cipher: Aes128) {
    pad(data, BLOCK_SIZE);
    for mut block in data.chunks_exact_mut(BLOCK_SIZE as usize) {
        cipher.encrypt_block(GenericArray::from_mut_slice(&mut block));
    }
}

pub fn encrypt<K>(data: &mut Vec<u8>, key: K)
where
    K: AsRef<[u8]>,
{
    let key = key.as_ref();
    let cipher = Aes128::new(GenericArray::from_slice(key));
    encrypt_with_cipher(data, cipher);
}
