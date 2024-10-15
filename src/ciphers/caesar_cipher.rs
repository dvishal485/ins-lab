use crate::{EncryptionAlgorithm, Text};

pub struct CaesarCipher {
    shift: usize,
}

impl CaesarCipher {
    pub fn new(shift: i32) -> Self {
        let shift = shift.rem_euclid(26) as usize;
        Self { shift }
    }
}

impl EncryptionAlgorithm for CaesarCipher {
    fn encrypt(&self, plain_text: &Text) -> Text {
        plain_text
            .chars()
            .map(|c| ((c as i16 - 'A' as i16 + self.shift as i16).rem_euclid(26) + 'A' as i16))
            .map(|s| s as u8 as char)
            .collect::<String>()
            .into()
    }

    fn decrypt(&self, cipher_text: &Text) -> Text {
        cipher_text
            .chars()
            .map(|c| ((c as i16 - 'A' as i16 - self.shift as i16).rem_euclid(26) + 'A' as i16))
            .map(|s| s as u8 as char)
            .collect::<String>()
            .into()
    }
}
