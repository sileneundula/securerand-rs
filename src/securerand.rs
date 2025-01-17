pub struct SecureRandom;

use crate::argon2id::FuscsinePasswordDerive;
use crate::rngs::FuschineCSPRNG;

impl SecureRandom {
    pub fn new(pass: &str) -> [u8;32] {
        let seed = FuscsinePasswordDerive::new(pass);
        let csprng = FuschineCSPRNG::from_seed_32(seed);
        return csprng
    }
}