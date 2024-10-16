use ins_lab::ciphers::rsa::{ArgumentsRSA, RSA};
use ins_lab::{encrypt_decrypt, intro};
pub const EXAMPLE_PLAIN_NUMBER: i64 = 88;

pub fn main() {
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
