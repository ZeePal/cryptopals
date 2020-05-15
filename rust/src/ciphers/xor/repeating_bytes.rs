pub fn crypt<K: AsRef<[u8]>, T: AsRef<[u8]>>(key: K, data: T) -> Vec<u8> {
    let key = key.as_ref();
    let data = data.as_ref();
    data.iter()
        .zip(key.iter().cycle()) // Set the key's iterator to repeat/loop forever
        .map(|(&x, &y)| &x ^ &y)
        .collect()
}
