use super::{EncryptionAlgorithm, Text};
use ndarray::{Array1, Array2};

pub struct HillCipher {
    key: Array2<i32>,
    inverse_key: Array2<i32>,
}

trait Mod26 {
    fn matrix_determinant(&self) -> i32;
    fn inverse_number(&self, num: i32) -> Result<i32, ()>;
    fn inverse_matrix(&self) -> Result<Self, ()>
    where
        Self: Sized;
}

impl Mod26 for Array2<i32> {
    fn matrix_determinant(&self) -> i32 {
        assert!(self.shape() == &[3, 3], "Matrix must be 3x3");
        let e = |i: usize, j: usize| (self.get((i, j)).unwrap());

        (e(0, 0) * (e(1, 1) * e(2, 2) - e(2, 1) * e(1, 2))
            - e(1, 0) * (e(0, 1) * e(2, 2) - e(2, 1) * e(0, 2))
            + e(2, 0) * (e(0, 1) * e(1, 2) - e(1, 1) * e(0, 2)))
        .rem_euclid(26)
    }
    fn inverse_number(&self, num: i32) -> Result<i32, ()> {
        if num % 2 == 0 || num % 13 == 0 {
            return Err(());
        }

        let inverse_mod26: [i32; 26] = [
            1, 0, 3, 0, 21, 0, 15, 0, 3, 0, 19, 0, 0, 0, 7, 0, 23, 0, 11, 0, 5, 0, 17, 0, 25, 0,
        ];

        let num = num.rem_euclid(26) as usize;
        Ok(inverse_mod26[num - 1])
    }
    fn inverse_matrix(&self) -> Result<Self, ()> {
        let det = self.matrix_determinant();
        let invdet = self.inverse_number(det)?;

        let mut inv_matrix = self.clone();

        macro_rules! minv {
            ($i:expr, $j:expr) => {
                inv_matrix.get_mut(($i, $j)).unwrap()
            };
        }

        let m = |i: usize, j: usize| (self.get((i, j)).unwrap());

        *minv!(0, 0) = m(1, 1) * m(2, 2) - m(2, 1) * m(1, 2);
        *minv!(0, 1) = m(0, 2) * m(2, 1) - m(0, 1) * m(2, 2);
        *minv!(0, 2) = m(0, 1) * m(1, 2) - m(0, 2) * m(1, 1);
        *minv!(1, 0) = m(1, 2) * m(2, 0) - m(1, 0) * m(2, 2);
        *minv!(1, 1) = m(0, 0) * m(2, 2) - m(0, 2) * m(2, 0);
        *minv!(1, 2) = m(1, 0) * m(0, 2) - m(0, 0) * m(1, 2);
        *minv!(2, 0) = m(1, 0) * m(2, 1) - m(2, 0) * m(1, 1);
        *minv!(2, 1) = m(2, 0) * m(0, 1) - m(0, 0) * m(2, 1);
        *minv!(2, 2) = m(0, 0) * m(1, 1) - m(1, 0) * m(0, 1);

        let inv_matrix = inv_matrix.mapv(|e| (e * invdet).rem_euclid(26));

        Ok(inv_matrix)
    }
}

impl HillCipher {
    pub fn new(key: &Text) -> Self {
        assert_eq!(key.len(), 9, "Key must be 9 characters long");

        let key = Array2::from_shape_vec(
            (3, 3),
            (key.chars().map(|c| c as i32 - 'A' as i32)).collect(),
        )
        .unwrap();

        let inverse_key = key.inverse_matrix().expect("Matrix is not invertible");
        HillCipher { key, inverse_key }
    }
    pub fn display_matrix(&self) {
        println!("Key Matrix:");
        println!("{}\n", self.key);
        println!("Inverse Key Matrix:");
        println!("{}\n", self.inverse_key);
    }
}

impl EncryptionAlgorithm for HillCipher {
    fn encrypt(&self, plain_text: &Text) -> Text {
        let plain_text = plain_text
            .chars()
            .map(|text| (text as u8 - 'A' as u8) as i32)
            .collect::<Vec<_>>();

        let plain_text = plain_text
            .chunks(3)
            .map(|text| {
                Array1::from_vec(
                    text.to_vec()
                        .into_iter()
                        .chain(std::iter::repeat(0))
                        .take(3)
                        .collect::<Vec<i32>>(),
                )
            })
            .collect::<Vec<_>>();

        plain_text
            .iter()
            .map(|text| self.key.clone().dot(text))
            .map(|text| text.mapv(|x| x.rem_euclid(26)))
            .map(|text| text.mapv(|x| (x + 'A' as i32) as u8 as char))
            .flatten()
            .collect::<String>()
            .into()
    }

    fn decrypt(&self, cipher_text: &Text) -> Text {
        assert!(
            cipher_text.len() % 3 == 0,
            "Cipher text length must be a multiple of 3"
        );

        let cipher_text = cipher_text
            .chars()
            .map(|text| (text as u8 - 'A' as u8) as i32)
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|text| Array1::from_iter(text.iter().cloned()))
            .collect::<Vec<_>>();

        cipher_text
            .iter()
            .map(|text| self.inverse_key.clone().dot(text))
            .map(|text| text.mapv(|x| x.rem_euclid(26)))
            .map(|text| text.mapv(|x| (x + 'A' as i32) as u8 as char))
            .flatten()
            .collect::<String>()
            .into()
    }
}
