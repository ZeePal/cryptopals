pub fn pad(data: &mut Vec<u8>, block_size: usize) {
    let pad = block_size - (data.len() % block_size);
    let mut padding = vec![pad as u8; pad];
    data.append(&mut padding);
}
