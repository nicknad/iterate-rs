// ── aead ─────────────────────────────────────────────────────────
pub(crate) const AES_GCM_NONCE_LEN: usize = 12;
pub(crate) const AES_GCM_TAG_LEN: usize = 16;

// ── kdf ─────────────────────────────────────────────────────────
pub(crate) const MASTER_KEY_LEN: usize = 32;
pub(crate) const ARGON2_MEMORY_COST: u32 = 64 * 1024;
pub(crate) const ARGON2_TIME_COST: u32 = 3;
pub(crate) const ARGON2_PARALLELISM: u32 = 4;
pub(crate) const ARGON2_SALT_LEN: usize = 16;
