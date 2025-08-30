use anyhow::{Result, anyhow};
use argon2::{Argon2, Algorithm, Params, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;
use crate::hashers::Hasher;

pub struct HasherArgon2idFast;
pub struct HasherArgon2idSecure;
pub struct HasherArgon2i;
pub struct HasherArgon2d;

impl Hasher for HasherArgon2idFast {
  fn hash(input: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(64 * 1024, 2, 1, None)
      .map_err(|e| anyhow!("Invalid Argon2 params: {}", e))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, argon2::Version::V0x13, params);

    let hash = argon2.hash_password(input.as_bytes(), &salt)
      .map_err(|e| anyhow!("Hashing error: {}", e))?
      .to_string();

    Ok(hash)
  }
}

impl Hasher for HasherArgon2idSecure {
  fn hash(input: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(256 * 1024, 4, 2, None)
      .map_err(|e| anyhow!("Invalid Argon2 params: {}", e))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, argon2::Version::V0x13, params);

    let hash = argon2.hash_password(input.as_bytes(), &salt)
      .map_err(|e| anyhow!("Hashing error: {}", e))?
      .to_string();

    Ok(hash)
  }
}

impl Hasher for HasherArgon2i {
  fn hash(input: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(128 * 1024, 3, 1, None)
      .map_err(|e| anyhow!("Invalid Argon2 params: {}", e))?;
    let argon2 = Argon2::new(Algorithm::Argon2i, argon2::Version::V0x13, params);

    let hash = argon2.hash_password(input.as_bytes(), &salt)
      .map_err(|e| anyhow!("Hashing error: {}", e))?
      .to_string();

    Ok(hash)
  }
}

impl Hasher for HasherArgon2d {
  fn hash(input: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(128 * 1024, 3, 1, None)
      .map_err(|e| anyhow!("Invalid Argon2 params: {}", e))?;
    let argon2 = Argon2::new(Algorithm::Argon2d, argon2::Version::V0x13, params);

    let hash = argon2.hash_password(input.as_bytes(), &salt)
      .map_err(|e| anyhow!("Hashing error: {}", e))?
      .to_string();

    Ok(hash)
  }
}