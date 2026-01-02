use crate::crypto::constants::{AES_GCM_NONCE_LEN, AES_GCM_TAG_LEN};
use crate::error::IterateError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum CryptoEnvelope {
    #[serde(rename = "1")]
    V1(AesGcmPacked) = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AesGcmPacked {
    pub nonce: [u8; AES_GCM_NONCE_LEN],
    pub ciphertext: Vec<u8>,
    pub tag: [u8; AES_GCM_TAG_LEN],
}

impl CryptoEnvelope {
    pub fn to_blob(&self) -> Result<Vec<u8>, IterateError> {
        postcard::to_stdvec(&self)
            .map_err(|e| IterateError::PostCardSerializationFailed(e.to_string()))
    }

    pub fn from_blob(blob: &[u8]) -> Result<Self, IterateError> {
        postcard::from_bytes(blob)
            .map_err(|e| IterateError::PostCardSerializationFailed(e.to_string()))
    }
}
