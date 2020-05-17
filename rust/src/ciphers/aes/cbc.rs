use std::error::Error;
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

pub fn decrypt_with_cipher<T: AsRef<[u8]>>(mut data: &mut Vec<u8>, cipher: Aes128, iv: T) {
    let mut last_block = iv.as_ref().to_vec();

    for mut block in data.chunks_exact_mut(16) {
        let orig_block = block.to_vec();
        cipher.decrypt_block(&mut GenericArray::from_mut_slice(&mut block));
        xor(&mut block, last_block);
        last_block = orig_block;
    }
    unpad(&mut data, 16);
}

pub fn decrypt<T: AsRef<[u8]>, X: AsRef<[u8]>>(mut data: &mut Vec<u8>, key: T, iv: X) {
    let cipher = Aes128::new(GenericArray::from_slice(key.as_ref()));
    decrypt_with_cipher(&mut data, cipher, iv);
}

pub fn decrypt_file<P: AsRef<Path>, T: AsRef<[u8]>, X: AsRef<[u8]>>(
    path: P,
    key: T,
    iv: X,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut output = b64_decode(
        file.bytes()
            .map(|x| x.unwrap())
            .filter(|&x| x != 10) // Ignore newline characters
            .collect::<Vec<u8>>(),
    )?;

    decrypt(&mut output, &key, &iv);
    Ok(output)
}

pub fn encrypt_with_cipher<T: AsRef<[u8]>>(mut data: &mut Vec<u8>, cipher: Aes128, iv: T) {
    let mut last_block = iv.as_ref();
    pad(&mut data, 16);

    for mut block in data.chunks_exact_mut(16) {
        xor(&mut block, last_block);
        cipher.encrypt_block(&mut GenericArray::from_mut_slice(&mut block));
        last_block = block;
    }
}

pub fn encrypt<T: AsRef<[u8]>, X: AsRef<[u8]>>(mut data: &mut Vec<u8>, key: T, iv: X) {
    let cipher = Aes128::new(GenericArray::from_slice(key.as_ref()));
    encrypt_with_cipher(&mut data, cipher, iv);
}
