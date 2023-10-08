use crypto_pals_rs::binary_vec::BinaryVec;
use crypto_pals_rs::scoring::score_str_frequency;

fn main() {
    let input = BinaryVec::try_from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();

    let mut values = Vec::with_capacity(u8::MAX as usize);

    for byte in u8::MIN..=u8::MAX {
        let xor = input.xor_byte(byte);
        let phrase = xor.to_string();
        let score = score_str_frequency(&phrase);

        values.push((score, phrase));
    }

    values.sort_by(|a, b| (*a).0.cmp(&(*b).0));

    for (i, (score, phrase)) in values.iter().enumerate() {
        println!("\n{i}: {score} : {phrase}");
    }
}