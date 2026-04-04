use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OxiditeConfig {
  pub base_path: PathBuf,
  pub java_path: String,
  pub max_parallel_downloads: usize,
  pub memory_gb: u8,
  pub extra_jvm_args: Vec<String>,
}

impl Default for OxiditeConfig {
  fn default() -> Self {
    Self {
      base_path: PathBuf::from("./minecraft.oxidite"),
      java_path: "java".to_string(),
      max_parallel_downloads: 10,
      memory_gb: 2,
      extra_jvm_args: vec![],
    }
  }
}