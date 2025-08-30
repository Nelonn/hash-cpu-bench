use std::{io::{self, Write}, time::Instant};
use clap::Parser;
use colored::*;
use crate::{benchmark::Benchmark, hashers::Preset};

mod hashers;
mod cpuinfo;
mod benchmark;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
  #[arg(long)]
  benchmark: bool,

  #[arg(long)]
  alg: Option<String>,

  #[arg(long)]
  repeats: Option<u16>,
}

fn main() {
  let args = Args::parse();
  let repeats = args.repeats.unwrap_or(15);

  println!(
    "\x1b]8;;{repo}\x1b\\{name}\x1b]8;;\x1b\\ v{version}",
    repo = env!("CARGO_PKG_REPOSITORY"),
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION").green()
  );

  // benchmark mode
  if args.benchmark {
    let mut benchmark = Benchmark::default();
    benchmark.start(repeats, args.alg);
    return;
  }

  // interactive mode
  let mut preset = if let Some(alg_name) = args.alg {
    match Preset::from_str(&alg_name) {
      Some(p) => {
        println!("{} {}{}", "Using algorithm preset".white(), p.as_ref().green(), "".normal());
        p
      }
      None => {
        println!("{} {}{}", "Unknown preset:".white(), alg_name.green(), "".normal());
        choose_preset()
      }
    }
  } else {
    choose_preset()
  };

  loop {
    print!("{} ", ">".white());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
      println!("{}", "Input error".white());
      continue;
    }

    let input = input.trim();

    if input.eq_ignore_ascii_case("exit") {
      break;
    }

    if input.starts_with("preset ") {
      let new_preset = input.strip_prefix("preset ").unwrap().trim();
      match Preset::from_str(new_preset) {
        Some(p) => {
          preset = p;
          println!("{} {}{}", "Preset changed to".white(), preset.as_ref().green(), "".normal());
        }
        None => println!("{} {}{}", "Unknown preset:".white(), new_preset.green(), "".normal()),
      }
      continue;
    }

    let start = Instant::now();
    match preset.hash(input) {
      Ok(hash) => {
        let duration = start.elapsed().as_millis();
        println!(
          "{}: {} ({}ms)",
          preset.as_ref().green().bold(),
          hash.white(),
          duration.to_string().green()
        );
      }
      Err(err) => println!("{} {}{}", "Failed to hash:".white(), err.to_string().green(), "".normal()),
    }
  }
}

fn choose_preset() -> Preset {
  let presets = Preset::to_vec();
  let presets_str = presets.iter()
    .map(|s| s.green().to_string())
    .collect::<Vec<_>>()
    .join(", ");

  println!("{} ({})", "Choose hashing preset".white(), presets_str);

  loop {
    print!("{}preset> ", "".white());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
      continue;
    }

    let input = input.trim();
    if let Some(p) = Preset::from_str(input) {
      return p;
    }

    println!("{} {}", "Unknown preset:".white(), input.green());
  }
}