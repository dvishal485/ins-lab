use crate::{EncryptionAlgorithm, Text};
use std::{collections::HashMap, ops::Deref};

pub struct PlayFair {
    matrix: Vec<Vec<char>>,
    indices: HashMap<char, (usize, usize)>,
}

impl PlayFair {
    pub fn new(mut key: Text) -> Self {
        key.push_str(&('A'..='Z').collect::<String>());

        let key = Text::from(key.replace('J', "I"));
        let key = key.unique();

        let matrix = key
            .chars()
            .collect::<Vec<_>>()
            .chunks(5)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<char>>>();

        assert!(
            matrix.len() == 5 && matrix.iter().all(|row| row.len() == 5),
            "Matrix generation was incorrect"
        );

        let indices = matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (c, (i, j))))
            .collect();

        PlayFair { matrix, indices }
    }
    pub fn display_matrix(&self) {
        self.matrix.iter().for_each(|row| println!("{:?}", row));
    }
    fn transform(&self, text: String, is_encrypt: bool) -> String {
        let mut cipher_text = String::new();
        let transform_index = |i: usize| {
            if is_encrypt {
                (i + 1) % 5
            } else {
                (i + (-1_i32).rem_euclid(5) as usize) % 5 // (i - 1) % 5
            }
        };
        text.chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 1 {
                    vec![chunk[0], 'X']
                } else {
                    chunk.to_vec()
                }
            })
            .for_each(|chunk| {
                let (a, b) = (chunk[0], chunk[1]);

                let (a_i, a_j) = self.indices[&a];
                let (b_i, b_j) = self.indices[&b];

                if a_i == b_i {
                    let c1_j = transform_index(a_j);
                    let c2_j = transform_index(b_j);
                    cipher_text.push(self.matrix[a_i][c1_j]);
                    cipher_text.push(self.matrix[b_i][c2_j]);
                } else if a_j == b_j {
                    let c1_i = transform_index(a_i);
                    let c2_i = transform_index(b_i);
                    cipher_text.push(self.matrix[c1_i][a_j]);
                    cipher_text.push(self.matrix[c2_i][b_j]);
                } else {
                    cipher_text.push(self.matrix[a_i][b_j]);
                    cipher_text.push(self.matrix[b_i][a_j]);
                }
            });

        cipher_text
    }
}

impl EncryptionAlgorithm for PlayFair {
    fn encrypt(&self, plain_text: &Text) -> Text {
        assert!(
            !plain_text.contains('X'),
            "Plain text cannot contain the character 'X'"
        );
        let plain_text = Text::from(plain_text.replace('J', "I"));

        let mut plain_text_chars = plain_text.chars().collect::<Vec<_>>();
        let mut i = 0;

        while i < plain_text_chars.len() {
            if i + 1 < plain_text_chars.len() && plain_text_chars[i] == plain_text_chars[i + 1] {
                plain_text_chars.insert(i + 1, 'X');
            }
            i += 2;
        }

        let plain_text = plain_text_chars.iter().collect::<String>();

        self.transform(plain_text, true).into()
    }

    fn decrypt(&self, cipher_text: &Text) -> Text {
        assert!(
            cipher_text.len() % 2 == 0,
            "Cipher text length must be even"
        );

        let mut decrypted_text = self.transform(cipher_text.deref().into(), false);
        decrypted_text.retain(|s| s != 'X');
        decrypted_text.into()
    }
}
