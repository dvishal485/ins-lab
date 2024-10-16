use ins_lab::intro;
use ins_lab::misc::diffie_hellman::Client;

pub fn main() {
    intro!("Diffie-Hellman Key Exchange");

    const PUBLIC_NUMBER_1: u64 = 1849;
    const PUBLIC_NUMBER_2: u64 = 91239;

    println!(
        "Decided public numbers: {}, {}\n",
        PUBLIC_NUMBER_1, PUBLIC_NUMBER_2
    );

    // decide private keys for each client
    let alice: Client<PUBLIC_NUMBER_1, PUBLIC_NUMBER_2> = Client::new(152983);
    let bob: Client<PUBLIC_NUMBER_1, PUBLIC_NUMBER_2> = Client::new(279923);

    println!("Alice {:#?}", alice);
    println!("Bob {:#?}", bob);
    println!("{}", "-".repeat(50));

    // generate keys
    let alice_generated_key = alice.generate_key();
    let bob_generated_key = bob.generate_key();

    println!("Alice's generated key: {}", alice_generated_key);
    println!("Bob's generated key: {}", bob_generated_key);
    println!("{}", "-".repeat(50));

    // generate secret keys
    let alice_secret_key = alice.generate_secret_key(bob_generated_key);
    let bob_secret_key = bob.generate_secret_key(alice_generated_key);

    println!("Alice's secret key: {}", alice_secret_key);
    println!("Bob's secret key: {}", bob_secret_key);
    println!("{}", "-".repeat(50));
}
