const COMMON_CHARS: &[u8] = b"AEIOUaeiou ";

pub fn count<D>(data: D) -> usize
where
    D: AsRef<[u8]>,
{
    let data = data.as_ref();
    data.iter().filter(|x| COMMON_CHARS.contains(x)).count()
}
