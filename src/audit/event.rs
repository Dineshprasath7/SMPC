// -----------------------------------------------------------------------------
// Audit Event
// -----------------------------------------------------------------------------

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AuditEvent {
    pub timestamp: String,
    pub operation_id: String,
    pub event: String,
    pub function: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ErrorEvent {
    pub timestamp: String,
    pub function: String,
    pub error: String,
}

impl AuditEvent {
    pub fn new(function: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            timestamp: utc_timestamp(),
            operation_id: Uuid::new_v4().to_string(),
            event: "function_called".to_string(),
            function: function.into(),
            parameters,
        }
    }
}

impl ErrorEvent {
    pub fn new(function: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            timestamp: utc_timestamp(),
            function: function.into(),
            error: error.into(),
        }
    }
}

// Timestamp conversion -> dependent free
// Example:
// 2026-10-05T03:23:42.123456Z
//
// If RFC3339 formatting is preferred, chrono/time can be used instead.
fn utc_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before UNIX epoch");

    format!("{}.{:09}Z", now.as_secs(), now.subsec_nanos())
}

/// Returns a SHA-256 fingerprint of secret material.
///
/// The actual secret is intentionally never written to the audit log.
pub fn secret_fingerprint(secret: &[u8]) -> String {
    let digest = Sha256::digest(secret);

    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
