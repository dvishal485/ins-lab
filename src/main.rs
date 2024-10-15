mod caesar_cipher;
mod hill_cipher;
mod playfair_cipher;
mod substituition_cipher;
mod text;

use text::{EncryptionAlgorithm, Text};

const EXAMPLE_PLAIN_TEXT: &str = "Hello beautifull";

fn main() {
    caesar_cipher_example::example();
    substituition_cipher_example::example();
    playfair_cipher_example::example();
    hill_cipher_example::example();
}

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

        encrypt_decrypt!(cipher);
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

        encrypt_decrypt!(cipher);
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

        let key = Text::from("ninjakill");
        println!("Key: {}", key);

        let cipher = PlayFair::new(key);
        cipher.display_matrix();

        encrypt_decrypt!(cipher);
    }
}

mod hill_cipher_example {
    use hill_cipher::HillCipher;

    use super::*;

    pub fn example() {
        println!(
            "{line}\n{cipher_name} Example\n{line}\n",
            line = "-".repeat(50),
            cipher_name = "Hill Cipher"
        );

        let key = Text::from("ninjakill");
        let cipher = HillCipher::new(&key);

        cipher.display_matrix();

        encrypt_decrypt!(cipher);
    }
}
