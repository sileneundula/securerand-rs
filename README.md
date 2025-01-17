# SecureRand-rs

## Description

**SecureRand-rs** is an easy to use crate for generating cryptographic randomness in different instances.

## Instance 1: The Password-Derived Randomness With External Input From The Operating System

The first implemented instance uses `argon2` and `rand_chacha`. It takes as input a password (UTF-8) and generates a cryptographic salt using the operating system, deriving a seed from the well trusted argon2 password hashing function.

It then inputs the 32-byte seed into `ChaCha20RNG` to generate cryptographic randomness.

**Note: Due to the salt being generated by the operating system, you can not use the same password to derive the same secret. In the future, this will be added.**

### Usage

```rust
use securerand_rs::securerand::SecureRandom;

fn main() {
    // Password for Argon2id
    let password: &str = "SecureRandPassword123";

    // SecureRandom Usage (Instance 1) using Argon2id, OS-CSPRNG Salt, and seeded into ChaCha20RNG. This password uses a generated salt so each time will be different.
    let csprng_bytes: [u8;32] = SecureRandom::new(password);
}
```

## Instance 2: The Password-With-Static-Salt Derived Randomness (Deterministic)

The second implementation uses a static salt with the password, deriving the key and randomness from the password along with the pregiven salt. This can be put in again and get the same value since it is used as a seed to get the ChaCha20RNG.

### Usage

```rust
use securerand_rs::securerand::SecureRandom;

fn main() {
    let password: &str = "SecureRandPasswordWithStaticSalt";

    let csprng_bytes: [u8;32] = SecureRandom::derive_from_password(password);
}
```

## Instance 3: The Password With Given Salt Derived Randomness (Deterministic)

In this case, the salt must be known to the user to regenerate it again. The salt should be random or deterministic if provided a system that is going to just derive from the password. You may want to generate randomness and then save the salt.

```rust
use securerand_rs::securerand::SecureRandom;

fn main() {
    // Password
    let password: &str = "SecureRandPasswordWithGivenSalt";
    // The salt can be anything. To be determenistic, you must save the salt.
    let salt: &str = "AnySalt"

    // If the salt and password are entered again, the same output will be determined
    let csprng_bytes: [u8;32] = SecureRandom::derive_from_password_and_salt(password,salt);
}
```

## Instance 4: Retrieving From Seed Using ChaCha20RNG

In this case, you can use the ChaCha20RNG to retrieve randomness from bytes={32,48,64,128}. It takes as input a seed of 32 bytes.

### Usage

```rust
use securerand_rs::rngs::FuschineCSPRNG;

fn main() {
    let seed: [u8;32] = [16u8;32];
    
    let csprng_32 = FuschineCSPRNG::from_seed_32(seed);
    let csprng_48 = FuschineCSPRNG::from_seed_48(seed);
    let csprng_64 = FuschineCSPRNG::from_seed_64(seed);
    let csprng_128 = FuschineCSPRNG::from_seed_128(seed);
}
```

### Instance 5: Retrieving From Operating System

We simply use the `getrandom` crate to retrieve bytes from the operating system. This crate is perfored if only getting operating system randomness.

### Usage

```rust
use securerand_rs::rngs::FuscineCSPRNG;

fn main() {
    let os_32 = FuscineCSPRNG::new_32();
    let os_64 = FuscineCSPRNG::get_64_bytes_from_os();
}
```

## TODO (or contributions):

- Add BIP39
- Add Output of More Bytes
- Add VRF
- Add Errors