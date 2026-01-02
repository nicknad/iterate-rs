use thiserror::Error;

/// Central error type for the Iterate application.
#[derive(Error, Debug)]
pub enum IterateError {
    // ── Database / Integrity ────────────────────────────────────────────────
    #[error("integrity table is missing or malformed")]
    MissingIntegrityRecord,

    #[error("database operation failed")]
    Database(#[from] rusqlite::Error),

    #[error("database integrity failed")]
    DatabaseIntegrity,

    #[error("Record was not found in database")]
    RecordNotFound,

    // ── Cryptography ────────────────────────────────────────────────────────
    #[error("password is incorrect or journal is corrupted")]
    InvalidPassword,

    #[error("AES-GCM authentication failed - possible tampering or wrong password")]
    AeadIntegrityFailure,

    #[error("key derivation failed (Argon2)")]
    KeyDerivationFailed,

    #[error("HKDF expansion failed")]
    HkdfExpansionFailed,

    #[error("decryption operation failed")]
    DecryptionFailed(String),

    // ── IO / Filesystem ─────────────────────────────────────────────────────
    #[error("I/O error while accessing journal file: {0}")]
    Io(#[from] std::io::Error),

    // ── Serialization / Configuration ──────────────────────────────────────
    #[error("failed to serialize/deserialize integrity parameters: {0}")]
    SerializationFailed(#[from] serde_json::Error),

    #[error("failed to serialize/deserialize: {0}")]
    PostCardSerializationFailed(String),

    // ── Internal / Programming errors (should never happen) ────────────────
    #[error("system random number generator failed — this is a critical security issue")]
    SystemRngFailure,

    #[error("Internal error: {0}")]
    Internal(String),
}
