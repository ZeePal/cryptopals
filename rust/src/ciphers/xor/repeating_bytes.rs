pub fn crypt<D: AsMut<[u8]>, T: AsRef<[u8]>>(data: &mut D, key: T) {
    let data = data.as_mut();
    let key = key.as_ref();

    for (a, b) in data.iter_mut().zip(key.iter().cycle()) {
        *a ^= b
    }
}
