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
use crate::utils::xor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const BLOCK_SIZE: u8 = 16;

pub fn decrypt_with_cipher<I>(data: &mut Vec<u8>, cipher: Aes128, iv: I)
where
    I: AsRef<[u8]>,
{
    let mut last_block = iv.as_ref().to_vec();

    for mut block in data.chunks_exact_mut(BLOCK_SIZE as usize) {
        let orig_block = block.to_vec();
        cipher.decrypt_block(GenericArray::from_mut_slice(&mut block));
        xor(&mut block, last_block);
        last_block = orig_block;
    }
    unpad(data, BLOCK_SIZE);
}

pub fn decrypt<K, I>(data: &mut Vec<u8>, key: K, iv: I)
where
    K: AsRef<[u8]>,
    I: AsRef<[u8]>,
{
    let key = key.as_ref();
    let cipher = Aes128::new(GenericArray::from_slice(key));
    decrypt_with_cipher(data, cipher, iv);
}

pub fn decrypt_file<P, K, I>(path: P, key: K, iv: I) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
    K: AsRef<[u8]>,
    I: AsRef<[u8]>,
{
    let file = BufReader::new(File::open(path)?);

    let mut output = b64_decode(
        file.bytes()
            .map(|x| x.unwrap())
            .filter(|&x| x != b'\n')
            .collect::<Vec<u8>>(),
    )?;

    decrypt(&mut output, key, iv);
    Ok(output)
}

pub fn encrypt_with_cipher<I>(data: &mut Vec<u8>, cipher: Aes128, iv: I)
where
    I: AsRef<[u8]>,
{
    let mut last_block = iv.as_ref();
    pad(data, BLOCK_SIZE);

    for mut block in data.chunks_exact_mut(BLOCK_SIZE as usize) {
        xor(&mut block, last_block);
        cipher.encrypt_block(GenericArray::from_mut_slice(&mut block));
        last_block = block;
    }
}

pub fn encrypt<K, I>(data: &mut Vec<u8>, key: K, iv: I)
where
    K: AsRef<[u8]>,
    I: AsRef<[u8]>,
{
    let key = key.as_ref();
    let cipher = Aes128::new(GenericArray::from_slice(key));
    encrypt_with_cipher(data, cipher, iv);
}
