use anyhow::Result;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, AsRefStr};

pub mod argon2;
pub mod bcrypt;

pub trait Hasher {
  fn hash(input: &str) -> Result<String>;
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr)]
pub enum Preset {
  Argon2idFast,         // m=64MB, t=2, p=1
  Argon2idSecure,   // m=256MB, t=4, p=2
  Argon2i,            // m=128MB, t=3, p=1
  Argon2d,            // m=128MB, t=3, p=1
  Bcrypt10,           // cost=10
  Bcrypt12,           // cost=12
  Bcrypt14,           // cost=14
}

impl Preset {
  pub fn to_vec() -> Vec<String> {
    Self::iter().map(|p| p.as_ref().to_string()).collect()
  }

  pub fn from_str(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
      "argon2idfast" => Some(Self::Argon2idFast),
      "argon2idsecure" => Some(Self::Argon2idSecure),
      "argon2i" => Some(Self::Argon2i),
      "argon2d" => Some(Self::Argon2d),
      "bcrypt10" => Some(Self::Bcrypt10),
      "bcrypt12" => Some(Self::Bcrypt12),
      "bcrypt14" => Some(Self::Bcrypt14),
      _ => None,
    }
  }

  pub fn hash(&self, input: &str) -> Result<String> {
    match self {
      Self::Argon2idFast => argon2::HasherArgon2idFast::hash(input),
      Self::Argon2idSecure => argon2::HasherArgon2idSecure::hash(input),
      Self::Argon2i => argon2::HasherArgon2i::hash(input),
      Self::Argon2d => argon2::HasherArgon2d::hash(input),
      Self::Bcrypt10 => bcrypt::HasherBcrypt10::hash(input),
      Self::Bcrypt12 => bcrypt::HasherBcrypt12::hash(input),
      Self::Bcrypt14 => bcrypt::HasherBcrypt14::hash(input),
    }
  }
}
