use rand::prelude::*;

use crate::ciphers::aes::cbc::encrypt as cbc_encrypt;
use crate::ciphers::aes::ecb::encrypt as ecb_encrypt;

const BLOCK_SIZE: usize = 16;
const PLAY_WITH_MIN: usize = 5;
const PLAY_WITH_MAX: usize = 11;

pub fn aes_ecb<D>(data: D) -> Vec<u8>
where
    D: AsRef<[u8]>,
{
    let data = data.as_ref();

    let mut rng = rand::thread_rng();
    let key = rng.gen::<[u8; BLOCK_SIZE]>();

    let mut output = play_with_data(&mut rng, data);
    ecb_encrypt(&mut output, key);
    output
}

pub fn aes_cbc<D>(data: D) -> Vec<u8>
where
    D: AsRef<[u8]>,
{
    let data = data.as_ref();

    let mut rng = rand::thread_rng();
    let key = rng.gen::<[u8; BLOCK_SIZE]>();
    let iv = rng.gen::<[u8; BLOCK_SIZE]>();

    let mut output = play_with_data(&mut rng, data);
    cbc_encrypt(&mut output, key, iv);
    output
}

fn play_with_data(rng: &mut ThreadRng, data: &[u8]) -> Vec<u8> {
    let prefix_size = rng.gen_range(PLAY_WITH_MIN, PLAY_WITH_MAX);
    let mut output: Vec<u8> = (0..prefix_size).map(|_| rng.gen()).collect();
    output.extend(data);

    let suffix_size = rng.gen_range(PLAY_WITH_MIN, PLAY_WITH_MAX);
    let suffix = (0..suffix_size).map(|_| rng.gen::<u8>());
    output.extend(suffix);
    output
}
