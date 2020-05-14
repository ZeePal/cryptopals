const COMMON_CHARS: &'static [u8] = "AEIOUaeiou ".as_bytes();

pub fn count<T: AsRef<[u8]>>(raw_data: T) -> usize {
    let data = raw_data.as_ref();
    data.iter().filter(|x| COMMON_CHARS.contains(x)).count()
}
