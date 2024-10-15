mod caesar_cipher;
mod playfair_cipher;
mod substituition_cipher;
mod text;

use text::{EncryptionAlgorithm, Text};

const EXAMPLE_PLAIN_TEXT: &str = "Hello beautiful";

fn main() {
    // caesar_cipher_example::example();
    substituition_cipher_example::example();
    // playfair_cipher_example::example();
}

mod caesar_cipher_example {
    use super::*;
    use crate::caesar_cipher::CaesarCipher;

    pub fn example() {
        println!(
            "{line}\n{cipher_name} Example\n{line}\n",
            line = "-".repeat(50),
            cipher_name = "Casear Cipher"
        );

        let cipher = CaesarCipher::new(3);

        let plain_text = Text::from(EXAMPLE_PLAIN_TEXT);
        println!("Plain text: {}", plain_text);

        let cipher_text = cipher.encrypt(&plain_text);
        println!("Cipher text: {}", cipher_text);

        let decrypted_text = cipher.decrypt(&cipher_text);
        println!("Decrypted text: {}", decrypted_text);
    }
}

mod substituition_cipher_example {
    use super::*;
    use crate::substituition_cipher::SubstitutionCipher;

    pub fn example() {
        println!(
            "{line}\n{cipher_name} Example\n{line}\n",
            line = "-".repeat(50),
            cipher_name = "Substitution Cipher"
        );

        let key = Text::from("QWERTYUIOPASDFGHJKLZXCVBNM");
        let cipher = SubstitutionCipher::new(&key);

        let plain_text = Text::from(EXAMPLE_PLAIN_TEXT);
        println!("Plain text: {}", plain_text);

        let cipher_text = cipher.encrypt(&plain_text);
        println!("Cipher text: {}", cipher_text);

        let decrypted_text = cipher.decrypt(&cipher_text);
        println!("Decrypted text: {}", decrypted_text);
    }
}

mod playfair_cipher_example {
    use super::*;
    use playfair_cipher::PlayFair;

    pub fn example() {
        println!(
            "{line}\n{cipher_name} Example\n{line}\n",
            line = "-".repeat(50),
            cipher_name = "Playfair Cipher"
        );

        let key = Text::from("watermelon");
        println!("Key: {}", key);

        let playfair = PlayFair::new(key);
        playfair.display_matrix();

        let plain_text = Text::from(EXAMPLE_PLAIN_TEXT);
        println!("Plain text: {}", plain_text);

        let cipher_text = playfair.encrypt(&plain_text);
        println!("Cipher text: {}", cipher_text);

        let decrypted_text = playfair.decrypt(&cipher_text);
        println!("Decrypted text: {}", decrypted_text);
    }
}
