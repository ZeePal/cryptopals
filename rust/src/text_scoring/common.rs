const COMMON_CHARS: &'static [u8] = &[
    ' ' as u8, 'A' as u8, 'E' as u8, 'I' as u8, 'O' as u8, 'U' as u8, 'a' as u8, 'e' as u8,
    'i' as u8, 'o' as u8, 'u' as u8,
];

pub fn count<T: AsRef<[u8]>>(raw_data: T) -> usize {
    let data = raw_data.as_ref();
    data.iter().filter(|x| COMMON_CHARS.contains(x)).count()
}
