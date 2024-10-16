use ins_lab::ciphers::{sha::SHA256, DigitalSignature, HillCipher};
use ins_lab::{intro, Text};

pub const EXAMPLE_PLAIN_TEXT: &str = "hello cute planet";

pub fn main() {
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
