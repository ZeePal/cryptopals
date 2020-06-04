pub const ALPHABET_SIZE: u8 = 26;

#[derive(PartialEq, Debug)]
pub enum Shift {
    Left,
    Right,
}

pub fn crypt<D>(mut data: D, mut key: u8, shift: Shift)
where
    D: AsMut<[u8]>,
{
    let data = data.as_mut();
    assert!(key <= ALPHABET_SIZE);

    if shift == Shift::Left {
        key = ALPHABET_SIZE - key
    }

    for a in data.iter_mut() {
        let (offset, letter) = match a {
            65..=90 => (65, *a - 65),  // Upper Case in ASCII
            97..=122 => (97, *a - 97), // Lower Case in ASCII
            _ => continue,
        };

        *a = ((letter + key) % ALPHABET_SIZE) + offset;
    }
}
