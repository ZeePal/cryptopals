pub fn pad(data: &mut Vec<u8>, block_size: u8) {
    let pad = (block_size as usize) - (data.len() % (block_size as usize));
    let mut padding = vec![pad as u8; pad];
    data.append(&mut padding);
}

pub fn unpad(data: &mut Vec<u8>, block_size: u8) -> bool {
    let last = match data.last() {
        Some(i) => i,
        None => return false,
    };
    if *last > block_size {
        return false;
    }

    let mut iter = data.iter().rev().skip(1);
    for _ in 1..*last {
        match iter.next() {
            Some(i) => {
                if i != last {
                    return false;
                }
            }
            None => return false,
        }
    }
    let new_size = data.len() - (*last as usize);
    data.truncate(new_size);

    true
}
