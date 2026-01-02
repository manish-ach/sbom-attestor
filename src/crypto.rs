use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;

use crate::{error::AttestorError, hash::Hash};

pub fn sign(hash: &Hash, sk: &SigningKey) -> Signature {
    sk.sign(hash)
}

pub fn verify(hash: &Hash, sig: &Signature, pk: &VerifyingKey) -> Result<(), AttestorError> {
    pk.verify(hash, sig)
        .map_err(|_| AttestorError::InvalidSignature)
}

pub fn generate_keypair() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}
