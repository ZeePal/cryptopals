pub fn crypt<D, K>(mut data: D, key: K)
where
    D: AsMut<[u8]>,
    K: AsRef<[u8]>,
{
    let data = data.as_mut();
    let key = key.as_ref();

    for (a, b) in data.iter_mut().zip(key.iter().cycle()) {
        *a ^= b
    }
}
