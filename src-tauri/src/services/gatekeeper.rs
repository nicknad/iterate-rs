use crate::crypto::aead::{decrypt, encrypt};
use crate::crypto::cryptoenvelope::CryptoEnvelope;
use crate::crypto::kdf::{KdfParams, derive_service_keys};
use crate::crypto::servicekeys::ServiceKeys;
use crate::error::IterateError;
use rand::TryRngCore;
use rusqlite::Connection;
use tracing::error;
use zeroize::Zeroizing;

pub fn initialize_key_store<R: TryRngCore>(
    rng: &mut R,
    conn: &mut Connection,
    password: &[u8],
) -> Result<ServiceKeys, IterateError> {
    let kdf_params = KdfParams::try_new(rng)?;
    let key_encryption_key = kdf_params.derive_key_encryption_key(password)?;
    let mut master_key_bytes = [0u8; 32];
    rng.try_fill_bytes(&mut master_key_bytes).map_err(|e| {
        error!("RngError: {}", e);
        IterateError::SystemRngFailure
    })?;
    let master_key = Zeroizing::new(master_key_bytes);
    let envelope = encrypt(
        rng,
        &key_encryption_key,
        master_key.as_ref(),
        b"master-key-wrapping-v1",
    )?;
    let kdf_blob = kdf_params.to_blob()?;
    let wrapped_key_blob = envelope.to_blob()?;

    conn.execute(
        "INSERT INTO key_store (id, kdf_params, wrapped_key, created_at_utc) 
         VALUES (1, ?, ?, strftime('%s','now'))",
        (kdf_blob, wrapped_key_blob),
    )?;

    derive_service_keys(&master_key)
}

pub fn verify_password(conn: &Connection, password: &[u8]) -> Result<ServiceKeys, IterateError> {
    let (kdf_blob, wrapped_key_blob): (Vec<u8>, Vec<u8>) = conn
        .query_row(
            "SELECT kdf_params, wrapped_key FROM key_store WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| IterateError::MissingIntegrityRecord)?;

    let kdf_params: KdfParams = KdfParams::from_bytes(&kdf_blob)?;

    let key_encryption_key = kdf_params.derive_key_encryption_key(password)?;

    let envelope = CryptoEnvelope::from_blob(&wrapped_key_blob)?;

    let master_key_vec = decrypt(&key_encryption_key, &envelope, b"master-key-wrapping-v1")
        .map_err(|_| IterateError::InvalidPassword)?;

    let master_key_arr: [u8; 32] = master_key_vec
        .try_into()
        .map_err(|_| IterateError::DecryptionFailed("Decrypted key length mismatch".into()))?;
    let master_key = Zeroizing::new(master_key_arr);

    derive_service_keys(&master_key)
}
