use anyhow::Result;
use bcrypt::hash;
use super::Hasher;

pub struct HasherBcrypt10;
pub struct HasherBcrypt12;
pub struct HasherBcrypt14;

impl Hasher for HasherBcrypt10 {
  fn hash(input: &str) -> Result<String> {
    let hash = hash(input, 10)?; // cost=10
    Ok(hash)
  }
}

impl Hasher for HasherBcrypt12 {
  fn hash(input: &str) -> Result<String> {
    let hash = hash(input, 12)?; // cost=12
    Ok(hash)
  }
}

impl Hasher for HasherBcrypt14 {
  fn hash(input: &str) -> Result<String> {
    let hash = hash(input, 14)?; // cost=14
    Ok(hash)
  }
}