mod playfair_cipher;
mod text;

use playfair_cipher::PlayFair;
use text::{EncryptionAlgorithm, Text};

fn main() {
    playfair_example::example();
}

mod playfair_example {
    use super::*;

    pub fn example() {
        let key = Text::from("watermelon");
        println!("Key: {}", key);

        let playfair = PlayFair::new(key);
        playfair.display_matrix();

        let plain_text = Text::from("Hello beautiful");
        println!("Plain text: {}", plain_text);

        let cipher_text = playfair.encrypt(&plain_text);
        println!("Cipher text: {}", cipher_text);

        let decrypted_text = playfair.decrypt(&cipher_text);
        println!("Decrypted text: {}", decrypted_text);
    }
}
