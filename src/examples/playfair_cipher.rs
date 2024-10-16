use crate::*;
use crate::examples::EXAMPLE_PLAIN_TEXT;
use ciphers::PlayFair;

pub fn example() {
    intro!("Playfair Cipher");

    let key = Text::from("ninjakill");
    println!("Key: {}", key);

    let cipher = PlayFair::new(key);
    cipher.display_matrix();

    encrypt_decrypt!(cipher);
}
