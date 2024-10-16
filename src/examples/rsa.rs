use crate::*;use crate::examples::EXAMPLE_PLAIN_NUMBER;
    use ciphers::rsa::{ArgumentsRSA, RSA};

    pub fn example() {
        intro!("RSA");

        let args = ArgumentsRSA {
            p: 317,
            q: 11,
            e: 17,
        };
        println!("{:?}\n", args);

        let cipher = RSA::new(args);

        cipher.display_keys();
        println!();

        encrypt_decrypt!(cipher, EXAMPLE_PLAIN_NUMBER);
    }