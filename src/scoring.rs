/// scores a utf-8 string by frequency
///
pub fn score_str_frequency(input: &str) -> u128 {
    input.as_bytes()
        .iter()
        .map(|c| score_u8_frequency(*c))
        .fold(0, |acc, e| acc + e as u128)
}

pub fn score_u8_frequency(input: u8) -> u16 {
    match input.to_ascii_lowercase() {
        letter if letter.is_ascii_alphabetic() => unsafe {
            *FREQUENCY_TABLE.get_unchecked((letter - b'a') as usize)
        },
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_str() {
        let input = "abcDEFghijklmnopqrstuv w x ! y z?";
        assert_eq!(score_str_frequency(input),
                   FREQUENCY_TABLE.iter().fold(0u128, |acc, e| acc + *e as u128));
    }

    #[test]
    fn test_score_u8() {
        let mut max_char = ' ';
        let mut max_score = u16::MIN;

        let mut min_char = ' ';
        let mut min_score = u16::MAX;

        for c in b'a'..=b'z' {
            let score = score_u8_frequency(c);

            if score < min_score {
                min_char = char::from(c);
                min_score = score;
            }
            if score > max_score {
                max_char = char::from(c);
                max_score = score;
            }
        }

        assert_eq!(max_score, *FREQUENCY_TABLE.iter().max().unwrap());
        assert_eq!(max_char, 'e');

        assert_eq!(min_score, *FREQUENCY_TABLE.iter().min().unwrap());
        assert_eq!(min_char, 'z');
    }
}

// source: https://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies.html
// relative frequencies. multiplied by 100 to get ints, and sorted in alphabetical order to make indexing faster
const FREQUENCY_TABLE: [u16; 26] = [812, 149, 271, 432, 1202, 230, 203, 592, 731, 010, 069, 398, 261, // A-M
    695, 768, 182, 011, 602, 628, 910, 288, 111, 209, 017, 211, 007]; // N-Z