/// # SecureRandom
/// 
/// ## Process of SecureRandom (Using Generated Salt)
/// 
/// ### 1. Argon2id
/// 
/// 1. Accepts as `input` a UTF-8 Password
/// 2. Generates a cryptographically random salt for Argon2id
/// 3. Derives 32-byte seed from the password and salt
/// 
/// ### 2. ChaCha20CSPRNG
/// 
/// 1. Accepts as input the seed in Argon2id
/// 2. Derives randomness from the seed using ChaCha20RNG
/// 3. Returns 32-bytes.
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