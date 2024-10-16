use ins_lab::ciphers::CaesarCipher;
use ins_lab::{encrypt_decrypt, intro};

const EXAMPLE_PLAIN_TEXT: &str = "hello cute planet";

pub fn main() {
    intro!("Caesar Cipher");
    let cipher = CaesarCipher::new(3);
    encrypt_decrypt!(cipher, EXAMPLE_PLAIN_TEXT);
}
