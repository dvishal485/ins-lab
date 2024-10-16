use super::sha::SHA;
use crate::{EncryptionAlgorithm, Text};
use std::marker::PhantomData;

pub struct DigitalSignature<T: EncryptionAlgorithm, K: SHA> {
    cipher: T,
    sha: PhantomData<K>,
}

impl<T, K> DigitalSignature<T, K>
where
    T: EncryptionAlgorithm,
    K: SHA,
{
    pub fn new(cipher: T) -> Self {
        Self {
            cipher,
            sha: PhantomData,
        }
    }

    pub fn sign(&self, plain_text: &Text) -> Text {
        self.encrypt(plain_text)
    }

    pub fn verify(&self, cipher_text: &Text) -> Text {
        self.decrypt(cipher_text)
    }
}

impl<T, K> EncryptionAlgorithm for DigitalSignature<T, K>
where
    T: EncryptionAlgorithm,
    K: SHA,
{
    fn encrypt(&self, plain_text: &Text) -> Text {
        let hash = K::hash(plain_text);
        let signature: String = self.cipher.encrypt(&hash.into()).into();

        let mut encrypted_data = self.cipher.encrypt(plain_text);

        encrypted_data.push('/');
        encrypted_data.push_str(&signature);

        println!("Signed with signature: {}", signature);

        encrypted_data
    }

    fn decrypt(&self, cipher_text: &Text) -> Text {
        let Some((encrypted_data, signature)) = cipher_text.split_once('/') else {
            panic!("Invalid signed data!");
        };

        let decrypted_data = self.cipher.decrypt(&Text::from(encrypted_data));
        let hash = self.cipher.decrypt(&Text::from(signature));

        let signed_hash = Text::new(&K::hash(&decrypted_data.clone().into()));

        if signed_hash.text.contains(&hash.text) {
            eprintln!("Signature verification failed!");
        } else {
            println!("Signature verification success!");
        }

        decrypted_data
    }
}
