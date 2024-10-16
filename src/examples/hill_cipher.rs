use crate::examples::EXAMPLE_PLAIN_TEXT;
use crate::*;
use ciphers::HillCipher;

pub fn example() {
    intro!("Hill Cipher");

    let key = Text::from("ninjakill");
    let cipher = HillCipher::new(&key);

    cipher.display_matrix();

    encrypt_decrypt!(cipher);
}
