pub fn pad(data: &mut Vec<u8>, block_size: u8) {
    let pad: u8 = block_size - (data.len() % (block_size as usize)) as u8;
    data.reserve(pad as usize);
    for _ in 0..pad {
        data.push(pad);
    }
}

pub fn unpad(data: &mut Vec<u8>, block_size: u8) -> bool {
    let pad = match data.last() {
        Some(i) => *i,
        None => return false, // Data is empty, nothing to pad
    };
    if pad > block_size {
        return false; // Last bytes value greater than block_size (invalid padding)
    }

    let mut iter = data.iter().rev().skip(1); // Skip the byte we already checked just above
    for _ in 1..pad {
        match iter.next() {
            Some(i) => {
                if *i != pad {
                    return false; // Byte isnt the same value as the last byte (invalid padding)
                }
            }
            None => return false, // Data not long enough for the padding it has (invalid padding)
        }
    }
    let new_size = data.len() - (pad as usize);
    data.truncate(new_size);
    true
}
