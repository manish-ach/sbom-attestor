# SBOM-Attestor

An SBOM attestor is the component in a software-supply-chain security pipeline that is responsible for creating, signing, and publishing an attestation that proves a given SBOM (Software Bill of Materials) is authentic, complete, and has not been tampered with since it was produced.

**note**: *The attestor is the “notary” that looks at your SBOM file, checks it against agreed-upon rules, and then wraps it in a cryptographically signed statement (the attestation) that downstream consumers can verify.*


Typical responsibilities of an SBOM attestor:

1. Ingest the raw SBOM produced by a scanner or build tool.
2. Validate it against policy (e.g., every image layer must be described, license field must be present, no critical components without hashes, etc.).
3. Sign the validated SBOM (and often a digest of the artifact it describes) with a private key or via a KMS-backed signer.
4. Store the resulting attestation (e.g., an in-toto attestation or a Sigstore “bundle”) in an OCI registry alongside the image or in a transparency log such as Rekor.
5. Make the public key (or certificate) discoverable so verifiers can check the signature later.

## Flow
1. Developer reviews / builds / deploys
2. Tool computes cryptographic hash of artifact
3. Developer signs hash with private key
4. Attestation is written to a blockchain
5. Anyone can verify:
    - artifact hash
    - signer identity
    - build chain integrity

# Timeline

## Sprint 0 — Foundation (1 week)
**Goal**: Be cryptographically and architecturally ready
**Learn**
  - Ed25519 signatures (not RSA)
  - SHA-256 / BLAKE3 hashing
  - Merkle trees (concept + implementation)
  - Blockchain basics (blocks, headers, difficulty, consensus)

**Build**
Rust crate:
```
hash_artifact(path) -> Hash
sign(hash, private_key)
verify(hash, signature, public_key)
```

**Deliverable**
CLI that signs a file and verifies it
