use std::{
    fmt::Display,
    ops::{Shl, Shr, ShrAssign},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bits<const N: usize> {
    pub(super) bits: [bool; N],
}
macro_rules! impl_from {
    ($($t:ty),*) => { $(
        impl<const N: usize> From<$t> for Bits<N> {
            fn from(value: $t) -> Self {
                let mut bits = Bits::new();
                let mut value = value;
                for i in (0..N).rev() {
                    bits.bits[i] = (value & 1) == 1;
                    value.shr_assign(1);
                }
                bits
            }
        }
    )* };
}

impl_from!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const N: usize> From<&[bool]> for Bits<N> {
    fn from(value: &[bool]) -> Self {
        assert!(
            value.len() <= N,
            "Bits should be copied from same length slice"
        );
        let mut bits = [false; N];
        (&mut bits[..value.len()]).copy_from_slice(&value);

        Bits { bits }
    }
}

impl<const N: usize, const K: usize> From<&Bits<K>> for Bits<N> {
    fn from(value: &Bits<K>) -> Self {
        Bits::from(value.bits.as_slice())
    }
}

// impl<const N: usize, T> From<T> for Bits<N>
// where
//     T: ShrAssign + BitAnd<Output = T> + Copy + Eq + From<u8>,
// {
//     fn from(value: T) -> Self {
//         let mut bits = Bits::new();
//         let mut value = value;
//         let one = T::from(1);
//         for i in (0..N).rev() {
//             bits.bits[i] = (value & one) == one;
//             value.shr_assign(one);
//         }
//         bits
//     }
// }

impl<const N: usize> Display for Bits<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in self.bits.iter() {
            write!(f, "{}", if *bit { 1 } else { 0 })?;
        }
        Ok(())
    }
}

impl<const N: usize> Bits<N> {
    pub const fn new() -> Self {
        if N == 0 {
            panic!("N should be greater than 0");
        }
        Self { bits: [false; N] }
    }
    pub const fn len(&self) -> usize {
        N
    }
    pub fn circular_left_shift(&self, shift: usize) -> Self {
        let mut num = self.clone();
        num.bits.rotate_left(shift);
        num
    }
    pub fn circular_right_shift(&self, shift: usize) -> Self {
        let mut num = self.clone();
        num.bits.rotate_right(shift);
        num
    }
}

impl<const N: usize> Shl<usize> for Bits<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        let mut num = Self::new();

        if rhs >= N {
            return num;
        }

        for i in 0..N - rhs {
            num.bits[i] = self.bits[i + rhs];
        }

        num
    }
}

impl<const N: usize> Shr<usize> for Bits<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        let mut num = Self::new();

        if rhs >= N {
            return num;
        }

        for i in (rhs..N).rev() {
            num.bits[i] = self.bits[i - rhs];
        }

        num
    }
}
