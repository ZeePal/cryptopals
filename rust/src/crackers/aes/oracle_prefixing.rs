use crate::oracles::aes::ecb::prefixing::Oracle;

const PADDING_BYTE: u8 = b' ';

pub fn crack(oracle: &Oracle) -> Result<Vec<u8>, &'static str> {
    let (block_size, data_size) = detect_block_size_and_data_size(&oracle);
    let mut output = Vec::with_capacity(data_size);

    while output.len() < data_size {
        let prefix = get_next_block_prefix(&output, block_size);
        let target = get_next_target_block(&oracle, &output, block_size);

        match find_target_blocks_last_byte(&oracle, target, prefix) {
            Ok(i) => output.push(i),
            Err(e) => return Err(e),
        }
    }

    Ok(output)
}

fn detect_block_size_and_data_size(oracle: &Oracle) -> (usize, usize) {
    let init_len = oracle.function([]).len();
    let mut new_len = init_len;

    let mut buffer = vec![];
    while new_len == init_len {
        buffer.push(PADDING_BYTE);
        new_len = oracle.function(&buffer).len();
    }

    let block_size = new_len - init_len;
    let data_size = init_len - buffer.len();
    (block_size, data_size)
}

fn get_next_target_block(oracle: &Oracle, found: &[u8], block_size: usize) -> Vec<u8> {
    let byte_index = found.len() % block_size;
    let padding = vec![PADDING_BYTE; block_size - byte_index - 1];

    let data = oracle.function(padding);

    let block_index = (found.len() / block_size) * block_size;
    data[block_index..block_index + block_size].to_vec()
}

fn get_next_block_prefix(found: &[u8], block_size: usize) -> Vec<u8> {
    let target_size = block_size - 1;
    let len = found.len();

    if len < target_size {
        let mut output = vec![PADDING_BYTE; target_size - len];
        output.extend(found);
        return output;
    }

    found[len - target_size..].to_vec()
}

fn find_target_blocks_last_byte(
    oracle: &Oracle,
    target: Vec<u8>,
    prefix: Vec<u8>,
) -> Result<u8, &'static str> {
    let mut block = prefix;
    block.push(PADDING_BYTE);

    let block_size = target.len();
    let last_byte = block_size - 1;

    for i in 0..=255 {
        block[last_byte] = i;
        if oracle.function(&block)[..block_size] == *target {
            return Ok(i);
        }
    }
    Err("Unable to find target block")
}
