use ins_lab::ciphers::PlayFair;
use ins_lab::{encrypt_decrypt, intro};

pub const EXAMPLE_PLAIN_TEXT: &str = "hello cute planet";

pub fn main() {
    intro!("Playfair Cipher");

    let key = Text::from("ninjakill");
    println!("Key: {}", key);

    let cipher = PlayFair::new(key);
    cipher.display_matrix();

    encrypt_decrypt!(cipher, EXAMPLE_PLAIN_TEXT);
}
