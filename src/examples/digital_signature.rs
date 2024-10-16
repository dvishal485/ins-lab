use examples::EXAMPLE_PLAIN_TEXT;

use crate::ciphers::{sha::SHA256, DigitalSignature, HillCipher};
use crate::*;

pub fn example() {
    intro!("Digital Signature");

    let key = Text::from("ninjakill");
    let cipher = HillCipher::new(&key);

    let digital_signature = DigitalSignature::<_, SHA256>::new(cipher);

    let text = Text::from(EXAMPLE_PLAIN_TEXT);
    println!("Text: {}", text);

    let signed_text = digital_signature.sign(&text);
    println!("Signature: {}", signed_text);

    let decrypted_text = digital_signature.verify(&signed_text);
    println!("Decrypted text: {}", decrypted_text);
}
