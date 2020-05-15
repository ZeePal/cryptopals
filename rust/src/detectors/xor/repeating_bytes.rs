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

pub fn detect_keysizes<T: AsRef<[u8]>>(
    data: T,
    min_size: usize,
    max_size: usize,
    samples: usize,
    top: usize,
) -> Vec<KeysizeScore> {
    let data = data.as_ref();

    let mut output = vec![];
    for size in min_size..=max_size {
        let mut score = 0;
        let mut checked = 0;
        let mut chunks = data.chunks_exact(size);
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
                size: size,
                score: score as f32 / checked as f32 / size as f32,
            },
        );
    }

    output
}
