use crate::utils::hamming_distance;

pub struct KeysizeScore {
    pub size: usize,
    pub score: f32,
}

fn add_keysize_score(list: &mut Vec<KeysizeScore>, max: usize, item: KeysizeScore) {
    let index = list
        .binary_search_by(|x| x.score.partial_cmp(&item.score).unwrap())
        .unwrap_or_else(|x| x);

    list.insert(index, item);
    list.truncate(max);
}

pub fn detect_keysizes<C>(
    cipher_text: C,
    min_size: usize,
    max_size: usize,
    samples: usize,
    top: usize,
) -> Vec<KeysizeScore>
where
    C: AsRef<[u8]>,
{
    let cipher_text = cipher_text.as_ref();

    let mut output = vec![];
    for keysize in min_size..=max_size {
        let mut score = 0;
        let mut checked = 0;

        let mut chunks = cipher_text.chunks_exact(keysize);
        while let (Some(x), Some(y)) = (chunks.next(), chunks.next()) {
            score += hamming_distance(x, y);
            checked += 1;
            if checked >= samples {
                break;
            }
        }

        add_keysize_score(
            &mut output,
            top,
            KeysizeScore {
                size: keysize,
                score: score as f32 / checked as f32 / keysize as f32,
            },
        );
    }
    output
}
