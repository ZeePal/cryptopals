use super::xor;

pub fn hamming_distance<L, R>(left: L, right: R) -> u32
where
    L: AsRef<[u8]>,
    R: AsRef<[u8]>,
{
    let mut left = left.as_ref().to_vec();
    let right = right.as_ref();

    xor(&mut left, right);
    left.iter().map(|&x| x.count_ones()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }
}
