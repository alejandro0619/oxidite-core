use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxiditeError {
  #[error("Network error: {0}")]
  Network(#[from] reqwest::Error),

  #[error("Files error (I/O): {0}")]
    Io(#[from] std::io::Error),

    #[error("Validation error (Hash incorrect): expected {expected}, found {found}")]
    HashMismatch { expected: String, found: String },

    #[error("Minecraft version not found: {0}")]
    VersionNotFound(String),

    #[error("Error in the manifest or metadata: {0}")]
    MetadataError(String),

    #[error("Java not found or incompatible version: {0}")]
    JavaError(String),

    #[error("Error al procesar JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error desconocido: {0}")]
    Unknown(String),
}

pub type OxiditeResult<T> = Result<T, OxiditeError>;

