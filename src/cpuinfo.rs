use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuInfo {
  pub name: String,
  pub frequency: u64,
}

impl Default for CpuInfo {
  fn default() -> Self {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu = sys.cpus().first().expect("No CPU found");
    CpuInfo {
      name: cpu.brand().to_string(),
      frequency: cpu.frequency(),
    }
  }
}