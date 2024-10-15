use std::ops::{Deref, DerefMut};

const START_CHARACTER: char = 'A';
const END_CHARACTER: char = 'Z';

#[derive(PartialEq)]
pub struct Text {
    pub text: String,
    pub number: u64,
}

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

impl From<Text> for String {
    fn from(val: Text) -> Self {
        if val.text.is_empty() {
            val.number.to_string()
        } else {
            val.text
        }
    }
}

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Text {
                fn from(value: $t) -> Self {
                    Text {
                        text: String::new(),
                        number: value as u64,
                    }
                }
            }
            impl Into<$t> for Text {
                fn into(self) -> $t {
                    self.number as $t
                }
            }
        )*
    };
}

impl_from!(i8, i16, i32, i64, u8, u16, u32, u64);

impl Text {
    pub fn new(text: &str) -> Self {
        let text: String = text
            .to_uppercase()
            .chars()
            .filter(|&s| s >= START_CHARACTER && s <= END_CHARACTER)
            .collect();

        if text.is_empty() {
            // maybe it's a number
            if let Ok(number) = text.parse::<u64>() {
                return Text {
                    text: String::new(),
                    number,
                };
            }

            panic!("Invalid text provided");
        }

        Text { text, number: 0 }
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
        Text { text, number: 0 }
    }
}

impl Deref for Text {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.text
    }
}

pub trait EncryptionAlgorithm {
    fn encrypt(&self, plain_text: &Text) -> Text;
    fn decrypt(&self, cipher_text: &Text) -> Text;
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.text.is_empty() {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{}", self.text)
        }
    }
}
