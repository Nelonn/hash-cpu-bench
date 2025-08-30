pub mod argon2;

pub trait Hasher {
  fn hash(str: &str) -> Result<String, Box<dyn std::error::Error>>;
}