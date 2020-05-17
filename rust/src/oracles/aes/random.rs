use rand::prelude::*;

use crate::ciphers::aes::cbc::encrypt as cbc_encrypt;
use crate::ciphers::aes::ecb::encrypt as ecb_encrypt;

pub fn aes_ecb<T: AsRef<[u8]>>(data: T) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key = rng.gen::<[u8; 16]>();

    let mut output = play_with_data(&mut rng, &data);
    ecb_encrypt(&mut output, key);

    output
}

pub fn aes_cbc<T: AsRef<[u8]>>(data: T) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key = rng.gen::<[u8; 16]>();
    let iv = rng.gen::<[u8; 16]>();

    let mut output = play_with_data(&mut rng, &data);
    cbc_encrypt(&mut output, key, iv);

    output
}

fn play_with_data<T: AsRef<[u8]>>(rng: &mut ThreadRng, data: T) -> Vec<u8> {
    let data = data.as_ref();

    let mut output: Vec<u8> = (0..rng.gen_range(5, 11)).map(|_| rng.gen()).collect();
    output.extend(data);
    let suffix: Vec<u8> = (0..rng.gen_range(5, 11)).map(|_| rng.gen()).collect();
    output.extend(suffix);

    output
}
