use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContinuumValidatorError {
    #[error("continuum:item_type unknown: {0}")]
    UnknownType(String),
    #[error("continuum:item_type={0} requires field {1}")]
    MissingField(String, String),
}

const REQUIRED_FIELDS: &[(&str, &[&str])] = &[
    ("crypto_mnemonic", &["continuum:network"]),
    ("crypto_private_key", &["continuum:network"]),
    ("passkey_export", &["continuum:service_name"]),
    ("recovery_codes", &["continuum:service_name"]),
    ("login", &[]),
    ("secure_note", &[]),
    ("card", &[]),
    ("identity", &[]),
    ("ssh_key", &[]),
];

pub fn validate_continuum_fields(
    fields: &[(&str, &str)],
) -> Result<(), ContinuumValidatorError> {
    let kv: HashMap<_, _> = fields.iter().cloned().collect();
    let Some(item_type) = kv.get("continuum:item_type") else {
        return Ok(());
    };

    let required = REQUIRED_FIELDS
        .iter()
        .find(|(candidate, _)| *candidate == *item_type)
        .ok_or_else(|| ContinuumValidatorError::UnknownType(item_type.to_string()))?
        .1;

    for required_field in required {
        if !kv.contains_key(required_field) {
            return Err(ContinuumValidatorError::MissingField(
                item_type.to_string(),
                required_field.to_string(),
            ));
        }
    }

    Ok(())
}
