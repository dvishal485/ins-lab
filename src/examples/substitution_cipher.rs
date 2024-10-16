use crate::ciphers::SubstitutionCipher;
use crate::examples::EXAMPLE_PLAIN_TEXT;
use crate::*;

pub fn example() {
    intro!("Substitution Cipher");

    let key = Text::from("QWERTYUIOPASDFGHJKLZXCVBNM");
    let cipher = SubstitutionCipher::new(&key);

    encrypt_decrypt!(cipher);
}
