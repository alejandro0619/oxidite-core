mod config;
mod error;
pub use config::*;
pub use error::*;
#[derive(Debug, Clone)]
pub struct ProgressReport {
    pub current: usize,
    pub total: usize,
    pub message: String,
}