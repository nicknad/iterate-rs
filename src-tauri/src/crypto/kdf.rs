use crate::crypto::constants::{
    ARGON2_MEMORY_COST, ARGON2_PARALLELISM, ARGON2_SALT_LEN, ARGON2_TIME_COST, MASTER_KEY_LEN,
};
use crate::crypto::servicekeys::ServiceKeys;
use crate::error::IterateError;
use argon2::{Algorithm, Argon2, Params, Version};
use hkdf::Hkdf;
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use zeroize::Zeroizing;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KdfParams {
    Argon2idV1 {
        salt: [u8; ARGON2_SALT_LEN],
        m_cost: u32,
        t_cost: u32,
        p_cost: u32,
    },
}

impl KdfParams {
    pub fn try_new<R: TryRngCore>(rng: &mut R) -> Result<Self, IterateError> {
        let mut salt = [0u8; ARGON2_SALT_LEN];
        rng.try_fill_bytes(&mut salt)
            .map_err(|_| IterateError::SystemRngFailure)?;

        Ok(Self::Argon2idV1 {
            salt,
            m_cost: ARGON2_MEMORY_COST,
            t_cost: ARGON2_TIME_COST,
            p_cost: ARGON2_PARALLELISM,
        })
    }

    /// Deserializes the KDF parameters from a binary BLOB.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, IterateError> {
        postcard::from_bytes(bytes)
            .map_err(|e| IterateError::PostCardSerializationFailed(e.to_string()))
    }

    /// Serializes the KDF parameters into a binary BLOB for storage.
    pub fn to_blob(&self) -> Result<Vec<u8>, IterateError> {
        postcard::to_stdvec(self)
            .map_err(|e| IterateError::PostCardSerializationFailed(e.to_string()))
    }

    pub fn derive_key_encryption_key(
        &self,
        password: &[u8],
    ) -> Result<Zeroizing<[u8; 32]>, IterateError> {
        match self {
            KdfParams::Argon2idV1 {
                salt,
                m_cost,
                t_cost,
                p_cost,
            } => {
                let params = Params::new(*m_cost, *t_cost, *p_cost, Some(MASTER_KEY_LEN))
                    .map_err(|_| IterateError::KeyDerivationFailed)?;

                let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
                let mut key = Zeroizing::new([0u8; MASTER_KEY_LEN]);

                argon
                    .hash_password_into(password, salt, &mut *key)
                    .map_err(|_| IterateError::KeyDerivationFailed)?;

                Ok(key)
            }
        }
    }
}

type IterateHkdf = Hkdf<Sha512>;
pub fn derive_service_keys(master_key: &Zeroizing<[u8; 32]>) -> Result<ServiceKeys, IterateError> {
    let hk = IterateHkdf::new(Some(b"iterate-journal-v1"), master_key.as_ref());

    let mut content_key = [0u8; MASTER_KEY_LEN];
    let mut meta_key = [0u8; MASTER_KEY_LEN];

    hk.expand(b"content-encryption-key", &mut content_key)
        .map_err(|_| IterateError::HkdfExpansionFailed)?;
    hk.expand(b"meta-verification-key", &mut meta_key)
        .map_err(|_| IterateError::HkdfExpansionFailed)?;

    Ok(ServiceKeys {
        content: Zeroizing::new(content_key),
        meta: Zeroizing::new(meta_key),
    })
}
