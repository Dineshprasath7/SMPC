use serde::Serialize;
use std::fs;
use std::path::Path;

pub fn save_json<T, P>(value: &T, path: P) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let json = serde_json::to_string_pretty(value)?;

    fs::write(path, json)?;

    Ok(())
}

pub fn save_yaml<T, P>(value: &T, path: P) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let yaml = serde_saphyr::to_string(value)?;

    fs::write(path, yaml)?;

    Ok(())
}
