use ins_lab::ciphers::HillCipher;
use ins_lab::{encrypt_decrypt, intro};

pub const EXAMPLE_PLAIN_TEXT: &str = "hello cute planet";

pub fn main() {
    intro!("Hill Cipher");

    let key = Text::from("ninjakill");
    let cipher = HillCipher::new(&key);

    cipher.display_matrix();
    encrypt_decrypt!(cipher, EXAMPLE_PLAIN_TEXT);
}
