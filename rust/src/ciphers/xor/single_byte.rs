pub fn crypt<T: AsRef<[u8]>>(key: u8, data: T) -> Vec<u8> {
    let data = data.as_ref();
    data.iter().map(|&x| x ^ key).collect()
}
