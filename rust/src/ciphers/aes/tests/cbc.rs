use super::super::cbc::*;

use rand::prelude::*;

#[test]
fn test() {
    let mut rng = rand::thread_rng();
    for i in 0..50 {
        let input: Vec<u8> = (0..i).map(|_| rng.gen()).collect();
        let key = rng.gen::<[u8; 16]>();
        let iv = rng.gen::<[u8; 16]>();
        let mut result = input.clone();

        encrypt(&mut result, key, iv);
        assert_ne!(result, input);

        decrypt(&mut result, key, iv);
        assert_eq!(result, input);
    }
}
