use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;
use crate::hashers::Hasher;
use std::error::Error;

pub struct Argon2Hasher;

impl Hasher for Argon2Hasher {
  fn hash(input: &str) -> Result<String, Box<dyn Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
      .hash_password(input.as_bytes(), &salt)
      .map_err(|e| e.to_string())?
      .to_string();

    Ok(hash)
  }
}
