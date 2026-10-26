use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::Serialize;
// use sha2::Digest;
// use uuid::Uuid;

use crate::audit::event::{AuditEvent, ErrorEvent};

// -----------------------------------------------------------------------------
// Audit Logger
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AuditLogger {
    root_path: PathBuf,
}

impl AuditLogger {
    /// Creates an audit logger rooted at `root`.
    ///
    /// The directory is created if it does not already exist.
    pub fn new(root: impl AsRef<Path>) -> io::Result<Self> {
        let home =
            dirs::home_dir().ok_or_else(|| io::Error::other("could not determine home directory"));
        let root_path = home?.join("audit_smpc");

        fs::create_dir_all(&root_path)?;

        Ok(Self { root_path })
    }

    /// Writes an audit event to:
    ///
    ///     ~/audit_smpc/audit.jsonl
    ///
    pub fn log_event(&self, event: AuditEvent) -> io::Result<()> {
        self.write_line("audit.jsonl", &event)
    }

    /// Writes an error event to:
    ///
    ///     ~/audit_smpc/error.jsonl
    ///
    pub fn log_error(&self, event: ErrorEvent) -> io::Result<()> {
        self.write_line("error.jsonl", &event)
    }

    /// Common implementation for writing structured events.
    fn write_line<T: Serialize>(&self, filename: &str, event: &T) -> io::Result<()> {
        let path = self.root_path.join(filename);
        println!("Path => {:?}", path);
        let serialized = serde_json::to_string(event).map_err(io::Error::other)?;

        let mut file = OpenOptions::new().create(true).append(true).open(path)?;

        writeln!(file, "{serialized}")?;

        // Make the record durable before returning.
        file.sync_data()?;

        Ok(())
    }

    pub fn root_path(&self) -> &Path {
        &self.root_path
    }
}
