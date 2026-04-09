use crate::consensus::ValidatorState;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const VALIDATORS_FILE: &str = "validators.json";

/// Load validators from persistent storage
pub fn load_validators<P: AsRef<Path>>(data_dir: P) -> Result<HashMap<String, ValidatorState>, String> {
    let validators_path = data_dir.as_ref().join(VALIDATORS_FILE);

    if !validators_path.exists() {
        return Ok(HashMap::new());
    }

    let data = fs::read_to_string(&validators_path)
        .map_err(|e| format!("Failed to read validators file: {}", e))?;

    serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse validators file: {}", e))
}

/// Save validators to persistent storage
pub fn save_validators<P: AsRef<Path>>(
    data_dir: P,
    validators: &HashMap<String, ValidatorState>,
) -> Result<(), String> {
    let data_dir = data_dir.as_ref();

    // Ensure directory exists
    fs::create_dir_all(data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    let validators_path = data_dir.join(VALIDATORS_FILE);

    let data = serde_json::to_string_pretty(validators)
        .map_err(|e| format!("Failed to serialize validators: {}", e))?;

    fs::write(&validators_path, data)
        .map_err(|e| format!("Failed to write validators file: {}", e))?;

    tracing::info!("Validators persisted to {}", validators_path.display());
    Ok(())
}

/// Add or update a validator in persistent storage
pub fn add_validator<P: AsRef<Path>>(
    data_dir: P,
    address: String,
    stake: u64,
    mobile: bool,
) -> Result<(), String> {
    let mut validators = load_validators(&data_dir)?;

    validators.insert(
        address.clone(),
        ValidatorState {
            address: address.clone(),
            stake,
            is_mobile: mobile,
            last_proposal: 0,
            total_proposals: 0,
            total_validations: 0,
            uptime_percentage: 100.0,
            jailed: false,
        },
    );

    save_validators(data_dir, &validators)?;
    tracing::info!("Validator {} persisted with stake {}", address, stake);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load_validators() {
        let temp_dir = TempDir::new().unwrap();
        let mut validators = HashMap::new();

        validators.insert(
            "validator1".to_string(),
            ValidatorState {
                address: "validator1".to_string(),
                stake: 1000,
                is_mobile: false,
                last_proposal: 0,
                total_proposals: 0,
                total_validations: 0,
                uptime_percentage: 100.0,
                jailed: false,
            },
        );

        save_validators(temp_dir.path(), &validators).unwrap();

        let loaded = load_validators(temp_dir.path()).unwrap();
        assert_eq!(loaded.len(), 1);
        assert!(loaded.contains_key("validator1"));
    }

    #[test]
    fn test_empty_validators_dir() {
        let temp_dir = TempDir::new().unwrap();
        let loaded = load_validators(temp_dir.path()).unwrap();
        assert!(loaded.is_empty());
    }
}
