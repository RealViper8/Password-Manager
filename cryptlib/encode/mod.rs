#![allow(unused)]
extern crate bcrypt;

use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub struct Encoder {
    pub text: String,
    pub hash_cost: u32,
}

impl Default for Encoder {
    fn default() -> Self {
        Self {
            text: String::new(),
            hash_cost: DEFAULT_COST,
        }
    }
}

impl Encoder {
    pub fn hash(&self) -> BcryptResult<String> {
        hash(&self.text, self.hash_cost)
    }
    pub fn verify(&self, hash: &str) -> bool {
        verify(&self.text, hash).unwrap_or(false)
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
