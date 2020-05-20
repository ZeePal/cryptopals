use aesni::block_cipher_trait::generic_array::GenericArray;
use aesni::block_cipher_trait::BlockCipher;
use aesni::Aes128;
use rand::random;

use crate::ciphers::aes::ecb::encrypt_with_cipher;

const BLOCK_SIZE: usize = 16;

pub struct Oracle {
    cipher: Aes128,
    suffix: Vec<u8>,
}

impl Oracle {
    pub fn new<S>(suffix: S) -> Oracle
    where
        S: AsRef<[u8]>,
    {
        let suffix = suffix.as_ref();

        let key = random::<[u8; BLOCK_SIZE]>();
        Oracle {
            cipher: Aes128::new(GenericArray::from_slice(&key)),
            suffix: suffix.to_vec(),
        }
    }

    pub fn function<P>(&self, prefix: P) -> Vec<u8>
    where
        P: AsRef<[u8]>,
    {
        let mut output = prefix.as_ref().to_vec();
        output.extend(&self.suffix);
        encrypt_with_cipher(&mut output, self.cipher);
        output
    }
}
