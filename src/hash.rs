use std::{fs::File, path::Path};

use sha2::{Digest, Sha256};

use crate::error::AttestorError;

pub type Hash = [u8; 32];

pub fn hash_artifact(path: &Path) -> Result<Hash, AttestorError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();

    std::io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize().into())
}
