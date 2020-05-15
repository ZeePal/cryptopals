use super::xor;

pub fn hamming_distance<T: AsRef<[u8]>, X: AsRef<[u8]>>(left: T, right: X) -> u32 {
    let left = left.as_ref();
    let right = right.as_ref();

    match xor(left, right) {
        Ok(r) => r.iter().map(|&x| x.count_ones()).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }
}
