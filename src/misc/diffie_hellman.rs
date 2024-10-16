use crate::meth::mod_pow;

#[derive(Debug)]
pub struct Client<const P: u64, const G: u64> {
    private_number: u64,
}

impl<const P: u64, const G: u64> Client<P, G> {
    pub fn new(private_number: u64) -> Self {
        Self { private_number }
    }

    pub fn generate_key(&self) -> u64 {
        mod_pow(G, self.private_number, P)
    }

    pub fn generate_secret_key(&self, other_client_generated_key: u64) -> u64 {
        mod_pow(other_client_generated_key, self.private_number, P)
    }
}
