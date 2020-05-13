pub fn crypt<T: AsRef<[u8]>>(key: u8, raw_data: T) -> Vec<u8> {
    let data = raw_data.as_ref();
    let length = data.len();

    let mut output = vec![0; length];
    for i in 0..length {
        output[i] = data[i] ^ key
    }

    output
}
