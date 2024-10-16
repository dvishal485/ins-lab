use crate::examples::EXAMPLE_PLAIN_TEXT;
use crate::*;
use ciphers::sha::{SHA, SHA1};

pub fn example() {
    intro!("SHA1");

    let text = Text::from(EXAMPLE_PLAIN_TEXT);
    println!("Text: {}", text);

    let hash = SHA1::hash(&text);
    println!("Hash: {}", hash);
}
