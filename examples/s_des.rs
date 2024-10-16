use ins_lab::{intro, misc::s_des::SDes};

pub fn main() {
    intro!("Simplified DES");

    let sdes = SDes::new(0b1010000010);
    let (key1, key2) = sdes.generate_keys();

    println!("{}", "-".repeat(50));
    println!("Simplified DES Key 1: \t{}", key1);
    println!("Simplified DES Key 2: \t{}", key2);
}
