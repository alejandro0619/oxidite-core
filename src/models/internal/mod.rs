#[derive(Debug, Clone)]
pub struct ProgressReport {
    pub current: usize,
    pub total: usize,
    pub message: String,
}