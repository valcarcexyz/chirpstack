use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};

// Return root certificates, optionally with the provided ca_file appended.
pub fn get_root_certs(ca_file: Option<String>) -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs()? {
        roots.add(cert)?;
    }

    if let Some(ca_file) = &ca_file {
        let f = File::open(ca_file).context("Open CA certificate")?;
        let mut reader = BufReader::new(f);
        let certs = rustls_pemfile::certs(&mut reader);
        for cert in certs.flatten() {
            roots.add(cert)?;
        }
    }

    Ok(roots)
}

pub fn private_key_to_pkcs8(pem: &str) -> Result<String> {
    if pem.contains("RSA PRIVATE KEY") {
        use rsa::{
            pkcs1::DecodeRsaPrivateKey,
            pkcs8::{EncodePrivateKey, LineEnding},
            RsaPrivateKey,
        };

        let pkey = RsaPrivateKey::from_pkcs1_pem(pem).context("Read RSA PKCS#1")?;
        let pkcs8_pem = pkey.to_pkcs8_pem(LineEnding::default())?;
        Ok(pkcs8_pem.as_str().to_owned())
    } else if pem.contains("EC PRIVATE KEY") {
        use elliptic_curve::{
            pkcs8::{EncodePrivateKey, LineEnding},
            SecretKey,
        };

        // We assume it is a P256 based secret-key, which is the most popular curve.
        // Attempting to decode it as P256 is still better than just failing to read it.
        let pkey: SecretKey<p256::NistP256> =
            SecretKey::from_sec1_pem(pem).context("Read EC SEC1")?;
        let pkcs8_pem = pkey.to_pkcs8_pem(LineEnding::default())?;
        Ok(pkcs8_pem.as_str().to_owned())
    } else {
        Ok(pem.to_string())
    }
}
