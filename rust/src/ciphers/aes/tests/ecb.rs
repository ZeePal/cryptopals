use super::super::ecb::*;

use rand::prelude::*;

#[test]
fn test() {
    let mut rng = rand::thread_rng();
    for i in 0..50 {
        let input: Vec<u8> = (0..i).map(|_| rng.gen()).collect();
        let key = rng.gen::<[u8; 16]>();
        let mut result = input.clone();

        encrypt(&mut result, key);
        assert_ne!(result, input);

        decrypt(&mut result, key);
        assert_eq!(result, input);
    }
}
