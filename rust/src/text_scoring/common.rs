use regex::bytes::Regex;

const COMMON_CHARS: &str = r"([EOeo ]|[AIUaiu ]\B)";

lazy_static! {
    static ref REGEX: Regex = Regex::new(COMMON_CHARS).unwrap();
}

pub fn count<D>(data: D) -> usize
where
    D: AsRef<[u8]>,
{
    let data = data.as_ref();

    REGEX.find_iter(data).count()
}
