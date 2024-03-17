use chrono::{DateTime, Utc};
use rsa::RsaPrivateKey;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FetchCertificatesError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Couldn't parse pkcs8 private key: {0}")]
    Pkcs8(#[from] rsa::pkcs8::Error),
}

/// A chat signing certificate.
#[derive(Clone, Debug)]
pub struct Certificates {
    /// The RSA private key.
    pub private_key: RsaPrivateKey,
    /// The RSA public key encoded as DER.
    pub public_key_der: Vec<u8>,

    pub signature_v1: Vec<u8>,
    pub signature_v2: Vec<u8>,

    pub expires_at: DateTime<Utc>,
    pub refresh_after: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CertificatesResponse {
    #[serde(rename = "keyPair")]
    pub key_pair: KeyPairResponse,

    /// base64 string; signed data
    #[serde(rename = "publicKeySignature")]
    pub public_key_signature: String,

    /// base64 string; signed data
    #[serde(rename = "publicKeySignatureV2")]
    pub public_key_signature_v2: String,

    /// Date like `2022-04-30T00:11:32.174783069Z`
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,

    /// Date like `2022-04-29T16:11:32.174783069Z`
    #[serde(rename = "refreshedAfter")]
    pub refreshed_after: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct KeyPairResponse {
    /// -----BEGIN RSA PRIVATE KEY-----
    /// ...
    /// -----END RSA PRIVATE KEY-----
    #[serde(rename = "privateKey")]
    pub private_key: String,

    /// -----BEGIN RSA PUBLIC KEY-----
    /// ...
    /// -----END RSA PUBLIC KEY-----
    #[serde(rename = "publicKey")]
    pub public_key: String,
}
