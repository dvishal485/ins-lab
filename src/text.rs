use std::ops::{Deref, DerefMut};

const START_CHARACTER: char = 'A';
const END_CHARACTER: char = 'Z';

#[derive(Debug)]
pub struct Text(pub String);

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text::new(&s)
    }
}

impl Text {
    pub fn new(text: &str) -> Self {
        Text(
            text.to_uppercase()
                .chars()
                .filter(|&s| s >= START_CHARACTER && s <= END_CHARACTER)
                .collect(),
        )
    }

    pub fn unique(&self) -> Self {
        assert!(
            START_CHARACTER <= END_CHARACTER,
            "Invalid range of characters provided"
        );
        let size = (END_CHARACTER as usize - START_CHARACTER as usize) + 1;
        let mut char_seen = vec![false; size];
        let text = self
            .chars()
            .filter(|&c| {
                let index = c as usize - START_CHARACTER as usize;
                if char_seen[index] {
                    false
                } else {
                    char_seen[index] = true;
                    true
                }
            })
            .collect();
        Text(text)
    }
}

impl Deref for Text {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait EncryptionAlgorithm {
    fn encrypt(&self, plain_text: &Text) -> Text;
    fn decrypt(&self, cipher_text: &Text) -> Text;
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}