use aesni::block_cipher_trait::generic_array::GenericArray;
use aesni::block_cipher_trait::BlockCipher;
use aesni::Aes128;
use rand::random;

use crate::ciphers::aes::ecb::encrypt_with_cipher;

pub struct Oracle {
    cipher: Aes128,
    suffix: Vec<u8>,
}

impl Oracle {
    pub fn new<T: AsRef<[u8]>>(suffix: T) -> Oracle {
        Oracle {
            cipher: Aes128::new(GenericArray::from_slice(&random::<[u8; 16]>())),
            suffix: suffix.as_ref().to_vec(),
        }
    }

    pub fn function<T: AsRef<[u8]>>(&self, prefix: T) -> Vec<u8> {
        let mut output = prefix.as_ref().to_vec();
        output.extend(&self.suffix);
        encrypt_with_cipher(&mut output, self.cipher);

        output
    }
}
