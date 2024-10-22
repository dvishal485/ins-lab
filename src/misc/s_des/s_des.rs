use super::Bits;

impl Bits<10> {
    pub fn p10(&self) -> Self {
        let mut num = [false; 10];
        const SHIFTERS: [usize; 10] = [3, 5, 2, 7, 4, 10, 1, 9, 8, 6];

        num.iter_mut()
            .zip(SHIFTERS.into_iter())
            .for_each(|(bit, shift)| *bit = self.bits[shift - 1]);

        Bits { bits: num }
    }
    pub fn p8(&self) -> Bits<8> {
        let mut num = [false; 8];
        const SHIFTERS: [usize; 8] = [6, 3, 7, 4, 8, 5, 10, 9];

        num.iter_mut()
            .zip(SHIFTERS.into_iter())
            .for_each(|(bit, shift)| *bit = self.bits[shift - 1]);

        Bits { bits: num }
    }
}

pub struct SDes {
    key: Bits<10>,
}

impl SDes {
    pub fn new<T>(key: T) -> Self
    where
        T: Into<Bits<10>>,
    {
        let key = key.into();
        Self { key }
    }
    pub fn generate_keys(&self) -> (Bits<8>, Bits<8>) {
        let p10 = self.key.p10();
        println!("Permutation (10): \t{}", p10);

        let (left, right) = p10.partition::<5, 5>();
        println!("L-R Partitions: \t{left} {right}");

        let left_shifted = left.circular_left_shift(1);
        let right_shifted = right.circular_left_shift(1);
        println!("Partitions Shifted: \t{left_shifted} {right_shifted}");

        let combined_key = left_shifted.combine(&right_shifted);
        println!("Combined Partitions: \t{}", combined_key);

        let key1 = combined_key.p8();
        println!("Permutation (8): \t{}\t(KEY 1)", key1);

        let left_shifted = left_shifted.circular_left_shift(2);
        let right_shifted = right_shifted.circular_left_shift(2);
        println!("Partitions Shifted: \t{left_shifted} {right_shifted}");

        let combined_key = left_shifted.combine(&right_shifted);
        println!("Combined Partitions: \t{}", combined_key);

        let key2 = combined_key.p8();
        println!("Permutation (8): \t{}\t(KEY 2)", key2);

        (key1, key2)
    }
}
