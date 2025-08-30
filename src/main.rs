use std::{io::{self, Write}, time::Instant};

use crate::hashers::{argon2::Argon2Hasher, Hasher};

mod hashers;

fn main() {
  println!("hash-time-checker v{}", env!("CARGO_PKG_VERSION"));

  loop {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_) => {
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
          break;
        }

        let instant = Instant::now();

        let hash = match Argon2Hasher::hash(input) {
          Ok(hash) => hash,
          Err(err) => {
            eprintln!("Unable to calculate hash of string `{input}`: {err}");
            continue;
          }
        };

        let duration = instant.elapsed().as_millis();

        println!("Hash: {hash}");
        println!("Calculate time: {} ms", duration);
      }
      Err(e) => println!("Input error: {}", e),
    }
  }
}