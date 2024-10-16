pub mod caesar_cipher;
pub mod playfair_cipher;
pub mod substitution_cipher;
pub mod hill_cipher;
pub mod rsa;
pub mod sha1;pub mod sha256;
pub mod digital_signature;

const EXAMPLE_PLAIN_TEXT: &str = "Hello beautifull";
const EXAMPLE_PLAIN_NUMBER: i64 = 88;



#[macro_export]
macro_rules! encrypt_decrypt {
    ($cipher:ident, $plain_text:ident) => {
        let plain_text = Text::from($plain_text);
        println!("Plain text: {}", plain_text);

        let cipher_text = $cipher.encrypt(&plain_text);
        println!("Cipher text: {}", cipher_text);

        let decrypted_text = $cipher.decrypt(&cipher_text);
        println!("Decrypted text: {}", decrypted_text);

        println!();
    };
    ($cipher:ident) => {
        encrypt_decrypt!($cipher, EXAMPLE_PLAIN_TEXT);
    };
}

#[macro_export]
macro_rules! intro {
    ($cipher: expr) => {
        println!(
            "{line}\n{cipher_name} Example\n{line}\n",
            line = "-".repeat(50),
            cipher_name = $cipher
        );
    };
}
