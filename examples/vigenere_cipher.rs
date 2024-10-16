use ins_lab::ciphers::VigenereCipher;
use ins_lab::{encrypt_decrypt, intro};
pub const EXAMPLE_PLAIN_TEXT: &str = "hello cute planet";

pub fn main() {
    intro!("Substitution Cipher");

    let key = Text::from("ninjakiller");
    let cipher = VigenereCipher::new(&key);

    encrypt_decrypt!(cipher, EXAMPLE_PLAIN_TEXT);
}
