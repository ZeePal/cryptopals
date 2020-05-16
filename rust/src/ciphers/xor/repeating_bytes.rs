pub fn crypt<T: AsRef<[u8]>>(data: &mut Vec<u8>, key: T) {
    let key = key.as_ref();

    for (a, b) in data.iter_mut().zip(key.iter().cycle()) {
        *a ^= b
    }
}
