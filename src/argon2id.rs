use argon2::Argon2;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
};

pub struct FuscsinePasswordDerive;

impl FuscsinePasswordDerive {
    pub fn new(password: &str) -> [u8;32] {
        // Generate Salt From OSRNG
        let salt = SaltString::generate(&mut OsRng);
        
        // Argon2id
        //let argon2 = Argon2::default();
        
        // Password Hash
        //let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();


        // Output Material
        let mut output: [u8;32] = [0u8; 32];

        // Argon2
        Argon2::default().hash_password_into(password.as_bytes(), salt.as_salt().as_str().as_bytes(), &mut output).unwrap();

        return output
    }
    pub fn new_with_salt(password: &str, salt: &str) -> [u8;32] {
        let mut output: [u8;32] = [0u8; 32];
        Argon2::default().hash_password_into(password.as_bytes(), salt.as_bytes(), &mut output).unwrap();

        return output
    }
    pub fn new_with_static_salt(password: &str) -> [u8;32] {
        let salt: &str = "SecureRandStaticSaltv1";
        let mut output: [u8;32] = [0u8;32];

        Argon2::default().hash_password_into(password.as_bytes(), salt.as_bytes(), &mut output).unwrap();

        return output
    }
}