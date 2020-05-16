pub fn crypt(data: &mut Vec<u8>, key: u8) {
    for a in data.iter_mut() {
        *a ^= key
    }
}
