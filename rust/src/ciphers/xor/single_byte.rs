pub fn crypt<D>(mut data: D, key: u8)
where
    D: AsMut<[u8]>,
{
    let data = data.as_mut();

    for a in data.iter_mut() {
        *a ^= key
    }
}
