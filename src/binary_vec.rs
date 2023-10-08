use std::convert::From;

/// Structure I'm using to represent arbitrary strings of binary data and the operations on them
///
///  # Members
/// * `data` - Stores the binary data to do various operations with
#[derive(Debug)]
pub struct BinaryVec {
    data: Vec::<u8>,
}

impl BinaryVec {
    /// Attempts to create a BinaryVec from the input. Requires that the input only contain valid characters
    ///
    /// # Argumenst
    /// * `input` - character array to try to convert into a BinaryVec
    pub fn try_from_hex(input: &str) -> Result<BinaryVec, BinaryVecParseError> {
        let mut data = Vec::<u8>::with_capacity(input.len());

        let mut even_parsed = true;

        let mut constructed_byte: u8 = 0;

        for c in input.chars() {
            let read_half_byte: u8 = match c {
                '0'..='9' => c as u8 - b'0',
                'a'..='f' => c as u8 - b'a' + 10,
                'A'..='F' => c as u8 - b'A' + 10,
                _ => return Err(BinaryVecParseError::ContainsInvalidChars),
            };
            even_parsed = !even_parsed;

            unsafe {
                constructed_byte = constructed_byte.unchecked_shl(4);
            }
            constructed_byte += read_half_byte;

            if even_parsed {
                data.push(constructed_byte);
            }
        };
        if !even_parsed {
            constructed_byte &= 0x0f;
            data.push(constructed_byte);
        }

        Ok(BinaryVec { data })
    }

    /// Converts into the hex string representation of the data
    pub fn to_hex_string(&self) -> String {
        self.data.iter().map(|byte| {
            format!("{:02x}", byte)
        }).collect()
    }

    pub fn to_string(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.data.clone())
        }
    }

    /// Calculates the xor result of BinaryVec
    ///
    /// # Arguments
    /// * `other` the BinaryVec to attempt to compute the xor with
    pub fn xor(&self, other: &Self) -> Result<BinaryVec, NonEqualDataLengths> {
        if self.data.len() != other.data.len() {
            Err(NonEqualDataLengths)
        } else {
            Ok(BinaryVec {
                data: self.data.iter()
                    .zip(other.data.iter())
                    .map(|val| val.0 ^ val.1)
                    .collect()
            })
        }
    }

    /// Calculates the xor result of BinaryVec with single byte
    ///
    /// # Arguments
    /// * `byte` the byte to compute xor with on the data stored
    pub fn xor_byte(&self, byte: u8) -> BinaryVec {
        BinaryVec {
            data: self.data.iter()
                .map(|val| *val ^ byte)
                .collect()
        }
    }

    /// Recovers the base64 version of the data stored
    pub fn to_base64_string(&self) -> String {
        let result_size: usize = (self.data.len() as f64 * (4.0f64 / 3.0)).ceil() as usize;
        let mut result = String::with_capacity(result_size);

        for chunk in self.data.chunks_exact(3) {
            unsafe {
                let chunk = [
                    *chunk.get_unchecked(0),
                    *chunk.get_unchecked(1),
                    *chunk.get_unchecked(2)
                ];
                let base64_chunk = process_full_chunk(&chunk);
                let base64_chunk = std::str::from_utf8_unchecked(&base64_chunk);

                result.push_str(base64_chunk);
            }
        }
        let excess_elements = self.data.len() % 3;
        if (excess_elements) != 0 {
            let chunk =
                process_partial_chunk(&self.data[self.data.len() - excess_elements..self.data.len()]);

            let base64_str;
            unsafe {
                base64_str = std::str::from_utf8_unchecked(&chunk);
            }

            result.push_str(base64_str);
        }

        result
    }
}

impl From<Vec<u8>> for BinaryVec {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

#[derive(Debug)]
pub enum BinaryVecParseError {
    ContainsInvalidChars
}

#[derive(Debug)]
pub struct NonEqualDataLengths;

/// Converts a full 3 byte chunk of data to base64
///
/// # arguments
/// * `input` 3 bytes to convert
fn process_full_chunk(input: &[u8; 3]) -> [u8; 4] {
    unsafe {
        let input = (*input.get_unchecked(0) as u32).unchecked_shl(8 * 2)
            | (*input.get_unchecked(1) as u32).unchecked_shl(8)
            | *input.get_unchecked(2) as u32;

        let idx_0 = input.unchecked_shr(6 * 3);
        let idx_1 = input.unchecked_shr(6 * 2) & MASK;
        let idx_2 = input.unchecked_shr(6) & MASK;
        let idx_3 = input & MASK;

        [
            *BASE_64_TABLE.get_unchecked(idx_0 as usize),
            *BASE_64_TABLE.get_unchecked(idx_1 as usize),
            *BASE_64_TABLE.get_unchecked(idx_2 as usize),
            *BASE_64_TABLE.get_unchecked(idx_3 as usize)
        ]
    }
}

/// gets a partial chunk into base64.
///
/// # Arguments
/// * `input` - data to convert to base64 string. Ignores characters outside `input[0..4]`
fn process_partial_chunk(input: &[u8]) -> [u8; 4] {
    let idx_0;
    let idx_1;
    let idx_2;
    let idx_3;

    let mut solution;

    unsafe {
        let mut bits: u32 = 0;

        if !input.is_empty() {
            bits |= (input[0] as u32).unchecked_shl(8 * 2);
        }
        if input.len() > 1 {
            bits |= (input[1] as u32).unchecked_shl(8);
        }
        if input.len() > 2 {
            bits |= input[2] as u32;
        }

        idx_0 = bits.unchecked_shr(6 * 3);
        idx_1 = bits.unchecked_shr(6 * 2) & MASK;
        idx_2 = bits.unchecked_shr(6) & MASK;
        idx_3 = bits & MASK;

        solution = [
            *BASE_64_TABLE.get_unchecked(idx_0 as usize),
            *BASE_64_TABLE.get_unchecked(idx_1 as usize),
            *BASE_64_TABLE.get_unchecked(idx_2 as usize),
            *BASE_64_TABLE.get_unchecked(idx_3 as usize)
        ];

        if input.len() < 3 {
            *solution.get_unchecked_mut(3) = b'=';
        }
        if input.len() < 2 {
            *solution.get_unchecked_mut(2) = b'=';
        }
        if input.is_empty() {
            *solution.get_unchecked_mut(1) = b'=';
            *solution.get_unchecked_mut(0) = b'=';
        }
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_inputs() {
        assert!(BinaryVec::try_from_hex("%").is_err());

        assert!(BinaryVec::try_from_hex("#00ff33").is_err());

        assert!(BinaryVec::try_from_hex("00ff33").is_ok());
        let v = BinaryVec::try_from_hex("00ff33").unwrap();
        assert_eq!(v.data, vec!(0, 255, 51));

        assert!(BinaryVec::try_from_hex("0123456789abcdef").is_ok());

        assert!(BinaryVec::try_from_hex("0123456789ABCEDF").is_ok());

        assert!(BinaryVec::try_from_hex("0 1 2 3").is_err());

        assert!(BinaryVec::try_from_hex("234 ").is_err());
    }

    #[test]
    fn from_u8_vec() {
        // Not really sure what to test here other than that the function exists.
        // I don't know of any vec u8's that would be failures... maybe I need to account for the last trailing u8 is required?
        let _ = BinaryVec::from(vec!());
        let _ = BinaryVec::from(vec!(0));
        let _ = BinaryVec::from(vec!(2, 15));
        let _ = BinaryVec::from(vec!(3, 16, 5));
        let _ = BinaryVec::from(vec!(3, 255, 5));
        let _ = BinaryVec::from(vec!(3, 255, 234));
        let _ = BinaryVec::from(vec!(3, 134, 234));
    }

    #[test]
    fn process_full_chunks() {
        assert_eq!(process_full_chunk(&[0, 0, 0]), [b'A', b'A', b'A', b'A']);
        assert_eq!(process_full_chunk(&[255, 255, 255]), [b'/', b'/', b'/', b'/']);
        assert_eq!(process_full_chunk(&[0, 0, 1]), [b'A', b'A', b'A', b'B']);
        assert_eq!(process_full_chunk(&[0, 0, 63]), [b'A', b'A', b'A', b'/']);
        assert_eq!(process_full_chunk(&[0, 1, 0]), [b'A', b'A', b'E', b'A']);
        assert_eq!(process_full_chunk(&[0, 63, 0]), [b'A', b'D', b'8', b'A']);
        assert_eq!(process_full_chunk(&[1, 0, 0]), [b'A', b'Q', b'A', b'A']);
        assert_eq!(process_full_chunk(&[63, 0, 0]), [b'P', b'w', b'A', b'A']);
    }

    #[test]
    fn process_partial_chunks() {
        assert_eq!(process_partial_chunk(&[0, 0, 0]), ([b'A', b'A', b'A', b'A']));
        assert_eq!(process_partial_chunk(&[255, 255, 255]), [b'/', b'/', b'/', b'/']);
        assert_eq!(process_partial_chunk(&[0, 0, 1]), ([b'A', b'A', b'A', b'B']));
        assert_eq!(process_partial_chunk(&[0, 0, 63]), ([b'A', b'A', b'A', b'/']));
        assert_eq!(process_partial_chunk(&[0, 1, 0]), ([b'A', b'A', b'E', b'A']));
        assert_eq!(process_partial_chunk(&[0, 63, 0]), ([b'A', b'D', b'8', b'A']));
        assert_eq!(process_partial_chunk(&[1, 0, 0]), ([b'A', b'Q', b'A', b'A']));
        assert_eq!(process_partial_chunk(&[63, 0, 0]), ([b'P', b'w', b'A', b'A']));

        assert_eq!(process_partial_chunk(&[0, 0]), ([b'A', b'A', b'A', b'=']));
        assert_eq!(process_partial_chunk(&[0, 1]), ([b'A', b'A', b'E', b'=']));
        assert_eq!(process_partial_chunk(&[0, 63]), ([b'A', b'D', b'8', b'=']));
        assert_eq!(process_partial_chunk(&[1, 0]), ([b'A', b'Q', b'A', b'=']));
        assert_eq!(process_partial_chunk(&[63, 0]), ([b'P', b'w', b'A', b'=']));

        assert_eq!(process_partial_chunk(&[0]), ([b'A', b'A', b'=', b'=']));
        assert_eq!(process_partial_chunk(&[1]), ([b'A', b'Q', b'=', b'=']));
        assert_eq!(process_partial_chunk(&[63]), ([b'P', b'w', b'=', b'=']));
    }

    #[test]
    fn to_base64_string() {
        let val = BinaryVec::try_from_hex("").unwrap();
        assert_eq!(val.to_base64_string(), "");

        let val = BinaryVec::from(vec!(0, 0, 0));
        assert_eq!(val.to_base64_string(), "AAAA");

        let val = BinaryVec::from(vec!(0, 0, 1));
        assert_eq!(val.to_base64_string(), "AAAB");

        let val = BinaryVec::try_from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        assert_eq!(
            val.to_base64_string(),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn to_hex_string() {
        let val = BinaryVec::try_from_hex("").unwrap();
        assert_eq!(val.to_hex_string(), "");

        let val = BinaryVec::from(vec!(255, 0, 255));
        assert_eq!(val.to_hex_string(), "ff00ff");

        let val = BinaryVec::from(vec!(0, 0, 1));
        assert_eq!(val.to_hex_string(), "000001");
    }

    #[test]
    fn test_xor() {
        let val_a = BinaryVec::try_from_hex("1c0111001f010100061a024b53535009181c").unwrap();
        let val_b = BinaryVec::try_from_hex("686974207468652062756c6c277320657965").unwrap();
        let val_c = val_a.xor(&val_b).unwrap();
        assert_eq!(val_c.to_hex_string(),
                   "746865206b696420646f6e277420706c6179");

        let val_d = BinaryVec::try_from_hex("abc").unwrap();
        let val_e = val_a.xor(&val_d);
        assert!(val_e.is_err());
    }

    #[test]
    fn test_xor_byte() {
        let val_a = BinaryVec::try_from_hex("0000ff").unwrap();
        let val_b = val_a.xor_byte(0xff);
        assert_eq!(val_b.to_hex_string(), "ffff00");
    }
}


/// Table to convert from a 6bit number (as usize...) to ascii character for Base64
const BASE_64_TABLE: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', // 0-12
    b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', // 13-25
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', // 26-38
    b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', // 39-41
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', // 42-62
    b'+', b'/', // 63-64
];

const MASK: u32 = 0b11_1111;