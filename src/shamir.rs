use crate::{
    audit::{
        event::{AuditEvent, ErrorEvent, secret_fingerprint},
        log::AuditLogger,
    },
    maths::{Share, gf_div, gf_mul},
};
use rand::RngExt;
use std::io;
use std::path::Path;
// use std::time::SystemTime;
// use std::time::UNIX_EPOCH;

mod hex_vec {
    use serde::{Deserialize, Deserializer, Serializer};
    // use serde::{Deserializer, Serializer};

    pub fn serialize<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(data))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub struct Smpc {
    audit: AuditLogger,
}

impl Smpc {
    pub fn new(audit_root: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            audit: AuditLogger::new(audit_root)?,
        })
    }

    pub fn split_secret(&self, secret: &[u8], n: u8, k: u8) -> Vec<Share> {
        if k < 2 {
            let error_event = ErrorEvent::new("split_secret", "threshold k must be at least 2");

            let _ = self.audit.log_error(error_event);

            panic!("threshold k must be at least 2");
        }

        if k > n {
            let error_event = ErrorEvent::new(
                "split_secret",
                format!("threshold k ({k}) cannot be greater than number of shares n ({n})"),
            );

            let _ = self.audit.log_error(error_event);
            panic!("threshold k cannot be greater than n");
        }

        let audit_event = AuditEvent::new(
            "split_secret",
            serde_json::json!({
                "secret_length": secret.len(),
                "secret_sha256": secret_fingerprint(secret),
                "n": n,
                "k": k}),
        );
        if let Err(error) = self.audit.log_event(audit_event) {
            let error_event =
                ErrorEvent::new("split_secret", format!("audit logging failed: {error}"));

            let _ = self.audit.log_error(error_event);

            panic!("unable to record audit event");
        }

        let mut rng = rand::rng();

        let mut shares = (1..=n)
            .map(|x| Share {
                x,
                data: Vec::with_capacity(secret.len()),
            })
            .collect::<Vec<_>>();

        for &byte in secret {
            let coeffs: Vec<u8> = (0..k - 1).map(|_| rng.random::<u8>()).collect();

            for share in &mut shares {
                let mut y = *coeffs.last().unwrap();

                for c in coeffs[..coeffs.len() - 1].iter().rev() {
                    y = gf_mul(y, share.x) ^ *c;
                }

                y = gf_mul(y, share.x) ^ byte;

                share.data.push(y);
            }
        }
        shares
    }

    pub fn reconstruct_secret(&self, shares: &[Share]) -> Vec<u8> {
        if shares.is_empty() {
            let error_event = ErrorEvent::new("reconstruct_secret", "shares cannot be empty");

            let _ = self.audit.log_error(error_event);

            panic!("shares cannot be empty");
        }

        let audit_event = AuditEvent::new(
            "reconstruct_secret",
            serde_json::json!({
                "share_count": shares.len(),
            }),
        );

        if let Err(error) = self.audit.log_event(audit_event) {
            let error_event =
                ErrorEvent::new("split_secret", format!("audit logging failed: {error}"));

            let _ = self.audit.log_error(error_event);

            panic!("unable to record audit event");
        }

        let len = shares[0].data.len();
        let mut secret = Vec::with_capacity(len);

        for byte in 0..len {
            let mut secret_byte = 0;

            for i in 0..shares.len() {
                let xi = shares[i].x;
                let yi = shares[i].data[byte];
                let mut numerator = 1;
                let mut denominator = 1;

                for j in 0..shares.len() {
                    if i == j {
                        continue;
                    }
                    let xj = shares[j].x;

                    numerator = gf_mul(numerator, xj);
                    denominator = gf_mul(denominator, xi ^ xj);
                }
                let lambda = gf_div(numerator, denominator);

                secret_byte ^= gf_mul(yi, lambda);
            }
            secret.push(secret_byte);
        }

        secret
    }
}
