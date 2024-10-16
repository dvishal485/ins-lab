use ins_lab::ciphers::sha::{SHA, SHA256};
use ins_lab::{intro, Text};

pub const EXAMPLE_PLAIN_TEXT: &str = "hello cute planet";

pub fn main() {
    intro!("SHA256");

    let text = Text::from(EXAMPLE_PLAIN_TEXT);
    println!("Text: {}", text);

    let hash = SHA256::hash(&text);
    println!("Hash: {}", hash);
}
