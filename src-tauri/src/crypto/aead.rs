use crate::crypto::constants::{AES_GCM_NONCE_LEN, AES_GCM_TAG_LEN};
use crate::crypto::cryptoenvelope::{AesGcmPacked, CryptoEnvelope};
use crate::error::IterateError;
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{AeadMutInPlace, KeyInit},
};
use rand::TryRngCore;
use zeroize::Zeroizing;

/// Encrypts the plaintext using AES-256-GCM with the provided key and AAD.
/// Returns (ciphertext, nonce, tag).
pub fn encrypt<R: TryRngCore>(
    rng: &mut R, // Inject RNG here
    key: &Zeroizing<[u8; 32]>,
    plaintext: &[u8],
    associated_data: &[u8],
) -> Result<CryptoEnvelope, IterateError> {
    let mut nonce = [0u8; AES_GCM_NONCE_LEN];
    rng.try_fill_bytes(&mut nonce)
        .map_err(|_| IterateError::SystemRngFailure)?;

    let mut cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_ref()));
    let mut buffer = plaintext.to_vec();

    let tag = cipher
        .encrypt_in_place_detached(Nonce::from_slice(&nonce), associated_data, &mut buffer)
        .map_err(|_| IterateError::AeadIntegrityFailure)?;

    let mut tag_arr = [0u8; AES_GCM_TAG_LEN];
    tag_arr.copy_from_slice(&tag);

    Ok(CryptoEnvelope::V1(AesGcmPacked {
        nonce,
        ciphertext: buffer,
        tag: tag_arr,
    }))
}

/// Encrypts the plaintext using AES-256-GCM with the provided key and AAD.
/// Returns serialized blob of CryptoEnvelope(version, ciphertext, nonce, tag).
pub fn decrypt(
    key: &Zeroizing<[u8; 32]>,
    envelope: &CryptoEnvelope,
    associated_data: &[u8],
) -> Result<Vec<u8>, IterateError> {
    match envelope {
        CryptoEnvelope::V1(data) => {
            let mut cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_ref()));
            let mut buffer = data.ciphertext.clone();

            cipher
                .decrypt_in_place_detached(
                    Nonce::from_slice(&data.nonce),
                    associated_data,
                    &mut buffer,
                    (&data.tag).into(),
                )
                .map_err(|_| IterateError::AeadIntegrityFailure)?;

            Ok(buffer)
        }
    }
}
