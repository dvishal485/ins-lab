use super::Bits;

impl Bits<10> {
    pub fn p10(&self) -> Self {
        let mut num = [false; 10];
        const SHIFTERS: [usize; 10] = [3, 5, 2, 7, 4, 10, 1, 9, 8, 6];

        for i in 0..10 {
            num[i] = self.bits[SHIFTERS[i] - 1];
        }

        Bits { bits: num }
    }
    pub fn p8(&self) -> Bits<8> {
        let mut num = [false; 8];
        const SHIFTERS: [usize; 8] = [6, 3, 7, 4, 8, 5, 10, 9];

        for i in 0..8 {
            num[i] = self.bits[SHIFTERS[i] - 1];
        }

        Bits { bits: num }
    }
    pub fn partition(&self) -> (Bits<5>, Bits<5>) {
        let (l, r) = self.bits.split_at(5);
        let (l, r): (&[bool; 5], &[bool; 5]) = (l.try_into().unwrap(), r.try_into().unwrap());
        (Bits::from(l), Bits::from(r))
    }
}

impl Bits<5> {
    pub fn combine(&self, rhs: &Bits<5>) -> Bits<10> {
        let mut num = Bits::from(self);
        for i in 5..10 {
            num.bits[i] = rhs.bits[i - 5];
        }
        num
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

        let (left, right) = p10.partition();
        println!("L-R Partitions: \t{left} {right}");
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
