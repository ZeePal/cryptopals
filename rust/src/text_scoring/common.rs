const COMMON_CHARS: &[u8] = b"AEIOUaeiou ";

pub fn count<T: AsRef<[u8]>>(raw_data: T) -> usize {
    let data = raw_data.as_ref();
    data.iter().filter(|x| COMMON_CHARS.contains(x)).count()
}
