use std::path::Path;

use ed25519_dalek::SigningKey;
use rand_core::OsRng;
use sbom_attestor::{
    crypto::{sign, verify},
    hash_artifact,
};

#[test]
fn sign_and_verify_file() {
    let sk = SigningKey::generate(&mut OsRng);
    let pk = sk.verifying_key();

    let hash = hash_artifact(Path::new("Cargo.toml")).unwrap();
    let sig = sign(&hash, &sk);

    assert!(verify(&hash, &sig, &pk).is_ok());
}
