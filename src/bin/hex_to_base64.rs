use crypto_pals_rs::binary_vec;

fn main() {
    binary_vec::BinaryVec::try_from_hex("a")
        .expect("known to pass at compile time");

    println!("Hello, world!");
}
