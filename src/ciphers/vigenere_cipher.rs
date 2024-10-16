use crate::ciphers::CaesarCipher;
use crate::{EncryptionAlgorithm, Text};

pub struct VigenereCipher {
    key: Vec<CaesarCipher>,
}
impl VigenereCipher {
    pub fn new(key: &Text) -> Self {
        let key = key
            .chars()
            .map(|c| c as u8 - b'A')
            .map(|shift| CaesarCipher::new(shift as i32))
            .collect();
        Self { key }
    }
}

impl EncryptionAlgorithm for VigenereCipher {
    fn encrypt(&self, plain_text: &Text) -> Text {
        plain_text
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                let cipher = &self.key[idx % self.key.len()];
                cipher.encrypt(&c.into()).chars().next().unwrap()
            })
            .collect::<String>()
            .into()
    }
    fn decrypt(&self, cipher_text: &Text) -> Text {
        cipher_text
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                let cipher = &self.key[idx % self.key.len()];
                cipher.decrypt(&c.into()).chars().next().unwrap()
            })
            .collect::<String>()
            .into()
    }
}
