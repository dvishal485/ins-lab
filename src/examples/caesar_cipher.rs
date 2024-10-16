pub use crate::text::{EncryptionAlgorithm, Text};
use crate::{ciphers::CaesarCipher, encrypt_decrypt, intro};
use crate::examples::EXAMPLE_PLAIN_TEXT;

pub fn example() {
    intro!("Caesar Cipher");

    let cipher = CaesarCipher::new(3);

    encrypt_decrypt!(cipher);
}
