pub mod ciphers;
pub mod meth;
pub mod text;
pub mod misc;
pub use text::{EncryptionAlgorithm, Text};

#[macro_export]
macro_rules! encrypt_decrypt {
    ($cipher:ident, $plain_text:ident) => {
        use ins_lab::{EncryptionAlgorithm, Text};
        let plain_text = Text::from($plain_text);
        println!("Plain text: {}", plain_text);

        let cipher_text = $cipher.encrypt(&plain_text);
        println!("Cipher text: {}", cipher_text);

        let decrypted_text = $cipher.decrypt(&cipher_text);
        println!("Decrypted text: {}", decrypted_text);

        println!();
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
