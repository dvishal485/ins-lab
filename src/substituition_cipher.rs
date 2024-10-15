use super::{EncryptionAlgorithm, Text};

pub struct SubstitutionCipher {
    decrypt_shift: Vec<char>,
    key: Vec<char>,
}

impl SubstitutionCipher {
    pub fn new(key: &Text) -> Self {
        assert!(
            key.len() == 26,
            "Key must be 26 characters long with only unique characters"
        );

        let mut decrypt_shift = vec!['A'; 26];

        key.chars()
            .enumerate()
            .map(|(idx, c)| ((idx as u8 + b'A') as char, c as usize - 'A' as usize))
            .for_each(|(idx, c)| decrypt_shift[c] = idx);

        let key: Vec<char> = key.chars().collect();

        Self { decrypt_shift, key }
    }
}

impl EncryptionAlgorithm for SubstitutionCipher {
    fn encrypt(&self, plain_text: &Text) -> Text {
        plain_text
            .chars()
            .map(|c| c as usize - 'A' as usize)
            .map(|i| self.key[i])
            .collect::<String>()
            .into()
    }

    fn decrypt(&self, cipher_text: &Text) -> Text {
        cipher_text
            .chars()
            .map(|c| c as usize - 'A' as usize)
            .map(|i| self.decrypt_shift[i])
            .collect::<String>()
            .into()
    }
}
