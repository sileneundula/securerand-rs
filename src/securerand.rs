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
    /// Derives From Password Using A Static Salt; Will Always Return The Same Randomness (Determeninistic)
    pub fn derive_from_password(pass: &str) -> [u8;32] {
        let seed = FuscsinePasswordDerive::new_with_static_salt(pass);
        let csprng = FuschineCSPRNG::from_seed_32(seed);
        return csprng
    }
    pub fn derive_from_password_and_salt(pass: &str, salt: &str) -> [u8;32] {
        let seed = FuscsinePasswordDerive::new_with_salt(pass, salt);
        let csprng = FuschineCSPRNG::from_seed_32(seed);
        return csprng
    }
}