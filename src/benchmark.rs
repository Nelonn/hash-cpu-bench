use std::{
  collections::HashMap,
  fs::File,
  io::{stdout, Write},
  thread,
  time::{Instant, SystemTime, UNIX_EPOCH},
};
use colored::*;
use crate::{cpuinfo::CpuInfo, hashers::Preset};
use serde::{Serialize, Deserialize};
use anyhow::Result;

const BENCHMARK_STR: &str = "examplestringForHash12312033$$$";
const SPINNER: [&str; 4] = ["▖", "▘", "▝", "▗"];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BenchmarkResult {
  pub average: u128,
  pub iterations: Vec<u128>,
}

impl BenchmarkResult {
  pub fn new() -> Self {
    Self {
      average: 0,
      iterations: vec![],
    }
  }

  pub fn add_iteration(&mut self, duration: u128) {
    self.iterations.push(duration);
    self.average = self.iterations.iter().sum::<u128>() / self.iterations.len() as u128;
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Benchmark {
  pub cpu: CpuInfo,
  pub results: HashMap<String, BenchmarkResult>,
}

impl Benchmark {
  fn run_algo(preset: &Preset, repeats: u16, line: u16) -> BenchmarkResult {
    let mut result = BenchmarkResult::new();

    let alg_color = preset.as_ref().green().bold();
    let number_color = " ".green();

    let spinner_done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let spinner_flag = spinner_done.clone();

    let spinner_line = line;
    thread::spawn(move || {
      let mut idx = 0;
      while !spinner_flag.load(std::sync::atomic::Ordering::Relaxed) {
        print!("\x1b[{};0H{} ", spinner_line, SPINNER[idx % SPINNER.len()].green());
        stdout().flush().unwrap();
        idx += 1;
        thread::sleep(std::time::Duration::from_millis(100));
      }
    });

    for i in 1..=repeats {
      let start = Instant::now();
      let _ = preset.hash(BENCHMARK_STR);
      let duration = start.elapsed().as_millis();
      result.add_iteration(duration);

      print!(
        "\x1b[{};2H {} ({}{}/{}{}) avg={}ms",
        line,
        alg_color,
        number_color,
        i,
        repeats,
        number_color,
        result.average,
      );
      stdout().flush().unwrap();
    }

    spinner_done.store(true, std::sync::atomic::Ordering::Relaxed);
    print!("\x1b[{};0H✔", line);
    stdout().flush().unwrap();
    println!();

    result
  }

  pub fn start(&mut self, repeats: u16, alg: Option<String>) {
    let presets: Vec<Preset> = if let Some(a) = alg {
      match Preset::from_str(&a) {
        Some(p) => vec![p],
        None => {
          println!("{} {}{}", "Preset not found:".white(), a.green(), "".normal());
          return;
        }
      }
    } else {
      Preset::to_vec()
        .iter()
        .filter_map(|s| Preset::from_str(s))
        .collect()
    };

    if presets.is_empty() {
      println!("{}", "No valid presets found, aborting benchmark.".white());
      return;
    }

    print!("\x1b[2J");
    stdout().flush().unwrap();

    let mut handles = vec![];

    for (idx, preset) in presets.iter().enumerate() {
      let preset_clone = *preset;
      let line = (idx + 1) as u16;
      let handle = thread::spawn(move || {
        let result = Benchmark::run_algo(&preset_clone, repeats, line);
        (preset_clone.as_ref().to_string(), result)
      });
      handles.push(handle);
    }

    for handle in handles {
      if let Ok((preset_name, result)) = handle.join() {
        self.results.insert(preset_name, result);
      }
    }

    match self.save() {
      Ok(f) => println!("\n{}", format!("Benchmark results saved to `{}`", f).white()),
      Err(err) => println!("\n{}", format!("Failed to save benchmark results: {}", err).white()),
    }
  }

  fn save(&self) -> Result<String> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let bench_file = format!("bench_{}.json", ts);
    let content = serde_json::to_string_pretty(&self)?;

    let mut file = File::create(&bench_file)?;
    file.write_all(content.as_bytes())?;
    Ok(bench_file)
  }
}
