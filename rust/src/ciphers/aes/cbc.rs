use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use base64::decode as b64_decode;

use aesni::block_cipher_trait::generic_array::GenericArray;
use aesni::block_cipher_trait::BlockCipher;
use aesni::Aes128;

use crate::utils::xor;

pub fn decrypt<D: AsMut<[u8]>, T: AsRef<[u8]>, X: AsRef<[u8]>>(data: &mut D, key: T, iv: X) {
    let data = data.as_mut();
    let mut last_block = iv.as_ref().to_vec();

    let cipher = Aes128::new(GenericArray::from_slice(key.as_ref()));
    for mut block in data.chunks_exact_mut(16) {
        let orig_block = block.to_vec();
        cipher.decrypt_block(&mut GenericArray::from_mut_slice(&mut block));
        xor(&mut block, last_block);
        last_block = orig_block;
    }
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
