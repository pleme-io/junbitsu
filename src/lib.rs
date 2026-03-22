use serde::{Deserialize, Serialize};

/// A single provisioning step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ProvisionStep {
    UnlockBootloader,
    FlashImage(String),
    InstallApk(String),
    SetSetting(String, String),
    LockBootloader,
}

/// A complete provisioning configuration for a device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisionConfig {
    pub device: String,
    pub steps: Vec<ProvisionStep>,
}

/// Validate a provisioning config, returning a list of error/warning strings.
#[must_use]
pub fn validate_config(config: &ProvisionConfig) -> Vec<String> {
    let mut errors = Vec::new();

    if config.device.is_empty() {
        errors.push("device name must not be empty".to_string());
    }

    if config.steps.is_empty() {
        errors.push("config must contain at least one provisioning step".to_string());
    }

    // Check that FlashImage occurs only after UnlockBootloader
    let has_unlock = config
        .steps
        .iter()
        .any(|s| matches!(s, ProvisionStep::UnlockBootloader));
    let has_flash = config
        .steps
        .iter()
        .any(|s| matches!(s, ProvisionStep::FlashImage(_)));

    if has_flash && !has_unlock {
        errors.push("FlashImage step present without prior UnlockBootloader step".to_string());
    }

    // Check for empty image paths
    for step in &config.steps {
        if let ProvisionStep::FlashImage(path) = step {
            if path.is_empty() {
                errors.push("FlashImage path must not be empty".to_string());
            }
        }
        if let ProvisionStep::InstallApk(path) = step {
            if path.is_empty() {
                errors.push("InstallApk path must not be empty".to_string());
            }
        }
    }

    errors
}

/// Parse a YAML string into a `ProvisionConfig`.
///
/// # Errors
///
/// Returns an error if the YAML is invalid or does not match the expected schema.
pub fn parse_config(yaml: &str) -> Result<ProvisionConfig, serde_yaml_ng::Error> {
    serde_yaml_ng::from_str(yaml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config_passes_validation() {
        let config = ProvisionConfig {
            device: "pixel8".to_string(),
            steps: vec![
                ProvisionStep::UnlockBootloader,
                ProvisionStep::FlashImage("system.img".to_string()),
                ProvisionStep::InstallApk("app.apk".to_string()),
                ProvisionStep::LockBootloader,
            ],
        };
        let errors = validate_config(&config);
        assert!(errors.is_empty(), "expected no errors, got: {errors:?}");
    }

    #[test]
    fn empty_steps_produces_error() {
        let config = ProvisionConfig {
            device: "pixel8".to_string(),
            steps: vec![],
        };
        let errors = validate_config(&config);
        assert!(errors.iter().any(|e| e.contains("at least one")));
    }

    #[test]
    fn flash_without_unlock_warns() {
        let config = ProvisionConfig {
            device: "pixel8".to_string(),
            steps: vec![ProvisionStep::FlashImage("boot.img".to_string())],
        };
        let errors = validate_config(&config);
        assert!(errors.iter().any(|e| e.contains("UnlockBootloader")));
    }

    #[test]
    fn parse_yaml_config() {
        let yaml = r#"
device: pixel8
steps:
  - type: UnlockBootloader
  - type: FlashImage
    value: system.img
  - type: InstallApk
    value: app.apk
  - type: SetSetting
    value:
      - "secure"
      - "install_non_market_apps=1"
  - type: LockBootloader
"#;
        let config = parse_config(yaml).expect("should parse");
        assert_eq!(config.device, "pixel8");
        assert_eq!(config.steps.len(), 5);
    }

    #[test]
    fn step_serialization_roundtrip() {
        let config = ProvisionConfig {
            device: "oriole".to_string(),
            steps: vec![
                ProvisionStep::UnlockBootloader,
                ProvisionStep::FlashImage("boot.img".to_string()),
                ProvisionStep::SetSetting("secure".to_string(), "1".to_string()),
                ProvisionStep::LockBootloader,
            ],
        };
        let json = serde_json::to_string(&config).unwrap();
        let deser: ProvisionConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deser);
    }
}
