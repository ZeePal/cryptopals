pub fn crypt<D: AsMut<[u8]>>(data: &mut D, key: u8) {
    let data = data.as_mut();

    for a in data.iter_mut() {
        *a ^= key
    }
}
