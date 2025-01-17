use argon2::Argon2;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
};

pub struct FuscsinePasswordDerive;

impl FuscsinePasswordDerive {
    pub fn new(password: &str) -> [u8;64] {
        // Generate Salt From OSRNG
        let salt = SaltString::generate(&mut OsRng);
        
        // Argon2id
        //let argon2 = Argon2::default();
        
        // Password Hash
        //let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();


        // Output Material
        let mut output: [u8;64] = [0u8; 64];

        // Argon2
        Argon2::default().hash_password_into(password.as_bytes(), salt.as_salt().as_str().as_bytes(), &mut output).unwrap();

        return output
    }
}