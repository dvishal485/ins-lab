use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;

use crate::Text;

pub struct SHA1 {}
pub struct SHA256 {}

impl SHA for SHA1 {
    fn hash(text: &Text) -> Text {
        let mut hasher = Sha1::new();
        hasher.input_str(text);
        hasher.result_str().into()
    }
}

impl SHA for SHA256 {
    fn hash(text: &Text) -> Text {
        let mut hasher = Sha256::new();
        hasher.input_str(text);
        hasher.result_str().into()
    }
}

pub trait SHA {
    fn hash(text: &Text) -> Text;
}
