use crate::meth::{gcd, mod_pow};

use crate::{EncryptionAlgorithm, Text};

pub struct RSA {
    private_key: u64,
    public_key: u64,
    n: u64,
}

#[derive(Debug)]
pub struct ArgumentsRSA {
    pub p: u64,
    pub q: u64,
    pub e: u64,
}

impl RSA {
    pub fn new(ArgumentsRSA { p, q, e }: ArgumentsRSA) -> Self {
        let n = p * q;
        let phi = (p - 1) * (q - 1);

        assert!(gcd(e, phi) == 1, "e and phi(n) must be coprime");

        let d = Self::mod_inverse(e, phi);

        Self {
            n,
            public_key: e,
            private_key: d,
        }
    }
    pub fn display_keys(&self) {
        println!("Public key: ({}, {})", self.n, self.public_key,);
        println!("Private key: ({}, {})", self.n, self.private_key);
    }
    fn mod_inverse(a: u64, b: u64) -> u64 {
        (1..).find(|d| a * d % b == 1).unwrap()
    }
}

impl EncryptionAlgorithm for RSA {
    fn encrypt(&self, plain_text: &Text) -> Text {
        let num = plain_text.number;
        let cipher_number = mod_pow(num as u64, self.public_key, self.n);

        Text::from(cipher_number)
    }

    fn decrypt(&self, cipher_text: &Text) -> Text {
        let num = cipher_text.number;
        let plain_number = mod_pow(num as u64, self.private_key, self.n);

        Text::from(plain_number)
    }
}
