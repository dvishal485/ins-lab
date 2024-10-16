pub mod ciphers;
mod examples;
pub mod meth;
pub mod text;
pub use text::{EncryptionAlgorithm, Text};

fn main() {
    examples::caesar_cipher::example();
    examples::substitution_cipher::example();
    examples::playfair_cipher::example();
    examples::hill_cipher::example();
    examples::rsa::example();
    examples::sha1::example();
    examples::sha256::example();
    examples::digital_signature::example();
}
