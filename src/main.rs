pub mod ciphers;
pub mod meth;
pub mod text;

pub use text::{EncryptionAlgorithm, Text};

const EXAMPLE_PLAIN_TEXT: &str = "Hello beautifull";
const EXAMPLE_PLAIN_NUMBER: i64 = 88;

fn main() {
    caesar_cipher_example::example();
    substituition_cipher_example::example();
    playfair_cipher_example::example();
    hill_cipher_example::example();
    rsa_example::example();
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
    use crate::ciphers::CaesarCipher;

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
    use crate::ciphers::SubstitutionCipher;

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
    use ciphers::PlayFair;

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
    use super::*;
    use ciphers::HillCipher;

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

mod rsa_example {
    use super::*;
    use ciphers::rsa::{ArgumentsRSA, RSA};

    pub fn example() {
        println!(
            "{line}\n{cipher_name} Example\n{line}\n",
            line = "-".repeat(50),
            cipher_name = "RSA"
        );

        let args = ArgumentsRSA { p: 317, q: 11, e: 17 };
        println!("{:?}\n", args);

        let cipher = RSA::new(args);

        cipher.display_keys();
        println!();

        encrypt_decrypt!(cipher, EXAMPLE_PLAIN_NUMBER);
    }
}
