#[cfg(feature = "rustls")]
pub use rustls::tls_config as rustls_config;

#[cfg(feature = "rustls")]
mod rustls {
    use crate::server::TlsConfig;
    use anyhow::Context;
    use pem::parse_many;
    use rust_tls::{Certificate, PrivateKey, ServerConfig};
    use rustls_pemfile::certs;
    use std::{fs::File, io::BufReader};

    /// Build a server config for rustls.
    pub fn tls_config(config: &dyn TlsConfig) -> anyhow::Result<ServerConfig> {
        let tls_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(config.verifier_rustls());

        let key = config
            .key_file()
            .ok_or_else(|| anyhow::anyhow!("TLS configuration error: Missing key file"))?;
        let cert = config
            .cert_bundle_file()
            .ok_or_else(|| anyhow::anyhow!("TLS configuration error: Missing cert file"))?;

        let cert_file = &mut BufReader::new(File::open(cert).unwrap());
        let cert_chain = certs(cert_file)
            .unwrap()
            .into_iter()
            .map(Certificate)
            .collect();

        let mut keys = Vec::new();

        let pems = std::fs::read(key)?;
        for pem in parse_many(pems)? {
            if pem.tag.contains("PRIVATE KEY") {
                keys.push(PrivateKey(pem.contents));
            }
        }

        if keys.len() > 1 {
            anyhow::bail!(
                "TLS configuration error: Found too many keys in the key file - found: {}",
                keys.len()
            );
        }

        let tls_config = if let Some(key) = keys.pop() {
            tls_config
                .with_single_cert(cert_chain, key)
                .context("Failed to set TLS certificate")?
        } else {
            anyhow::bail!("TLS configuration error: No key found in the key file")
        };

        Ok(tls_config)
    }
}

#[cfg(feature = "openssl")]
pub use openssl::tls_config as openssl_config;

#[cfg(feature = "openssl")]
mod openssl {
    use crate::server::TlsConfig;

    /// Build a server config for openssl.
    pub fn tls_config(config: &dyn TlsConfig) -> anyhow::Result<open_ssl::ssl::SslAcceptor> {
        let key = config
            .key_file()
            .ok_or_else(|| anyhow::anyhow!("TLS configuration error: Missing key file"))?;
        let cert = config
            .cert_bundle_file()
            .ok_or_else(|| anyhow::anyhow!("TLS configuration error: Missing cert file"))?;

        use open_ssl::ssl;
        let method = ssl::SslMethod::tls_server();
        let mut builder = ssl::SslAcceptor::mozilla_intermediate_v5(method)?;
        builder.set_private_key_file(key, ssl::SslFiletype::PEM)?;
        builder.set_certificate_chain_file(cert)?;
        // we ask for client certificates, but don't enforce them
        builder.set_verify_callback(ssl::SslVerifyMode::PEER, |_, ctx| {
            log::debug!(
                "Accepting client certificates: {:?}",
                ctx.current_cert()
                    .map(|cert| format!("{:?}", cert.subject_name()))
                    .unwrap_or_else(|| "<unknown>".into())
            );
            true
        });

        Ok(builder.build())
    }
}
