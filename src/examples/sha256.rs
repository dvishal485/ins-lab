use crate::*;
use ciphers::sha::{SHA, SHA256};
use examples::EXAMPLE_PLAIN_TEXT;

pub fn example() {
    intro!("SHA256");

    let text = Text::from(EXAMPLE_PLAIN_TEXT);
    println!("Text: {}", text);

    let hash = SHA256::hash(&text);
    println!("Hash: {}", hash);
}
