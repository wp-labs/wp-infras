use orion_conf::error::{ConfIOReason, StructError};
use serde::Deserialize;
use std::str::FromStr;
use wp_conf_base::structure::ConfStdOperation;
use wp_conf_base::{Protocol, Validate, de_bool_onoff};

#[test]
fn test_protocol_from_str() {
    // Test valid protocols
    assert_eq!(Protocol::from_str("tcp").unwrap(), Protocol::TCP);
    assert_eq!(Protocol::from_str("udp").unwrap(), Protocol::UDP);

    // Test case sensitivity (should fail)
    assert!(Protocol::from_str("TCP").is_err());
    assert!(Protocol::from_str("Tcp").is_err());

    // Test invalid protocol
    let result = Protocol::from_str("http");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Unsupported protocol")
    );
}

#[test]
fn test_protocol_display() {
    assert_eq!(format!("{}", Protocol::TCP), "tcp");
    assert_eq!(format!("{}", Protocol::UDP), "udp");
}

#[test]
fn test_de_bool_onoff() {
    #[derive(Debug, Deserialize)]
    struct TestConfig {
        #[serde(deserialize_with = "de_bool_onoff")]
        enabled: bool,
    }

    // Test various true values
    let test_cases = [
        ("true", true),
        ("1", true),
        ("on", true),
        ("yes", true),
        ("y", true),
        ("TRUE", true), // Case insensitive
        ("On", true),
    ];

    for (value, expected) in test_cases {
        let toml_str = format!("enabled = \"{}\"", value);
        let config: TestConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.enabled, expected, "Failed for value: {}", value);
    }

    // Raw integers should also work
    let config: TestConfig = toml::from_str("enabled = 1").unwrap();
    assert!(config.enabled);

    // Test various false values
    let test_cases = [
        ("false", false),
        ("0", false),
        ("off", false),
        ("no", false),
        ("n", false),
        ("FALSE", false), // Case insensitive
        ("OFF", false),
    ];

    for (value, expected) in test_cases {
        let toml_str = format!("enabled = \"{}\"", value);
        let config: TestConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.enabled, expected, "Failed for value: {}", value);
    }

    let config: TestConfig = toml::from_str("enabled = 0").unwrap();
    assert!(!config.enabled);

    // Test invalid values
    let test_cases = ["maybe", "2", "invalid"];

    for value in test_cases {
        let toml_str = format!("enabled = \"{}\"", value);
        let result: Result<TestConfig, _> = toml::from_str(&toml_str);
        assert!(result.is_err(), "Expected error for value: {}", value);
    }

    // Invalid integers should error
    for invalid in ["enabled = 2", "enabled = -1"] {
        let result: Result<TestConfig, _> = toml::from_str(invalid);
        assert!(result.is_err(), "Expected error for item: {}", invalid);
    }
}

#[test]
fn test_conf_std_operation_try_load() {
    #[derive(Debug, PartialEq)]
    struct FileBackedConf {
        marker: String,
    }

    impl ConfStdOperation for FileBackedConf {
        fn load(path: &str) -> orion_conf::error::OrionConfResult<Self> {
            let raw = std::fs::read_to_string(path).map_err(|err| {
                StructError::from(ConfIOReason::Other(format!("io error: {}", err)))
            })?;
            match raw.trim() {
                "ok" => Ok(FileBackedConf {
                    marker: "ok".to_string(),
                }),
                other => {
                    StructError::from(ConfIOReason::Other(format!("invalid content: {}", other)))
                        .err()
                }
            }
        }

        fn init(_: &str) -> orion_conf::error::OrionConfResult<Self> {
            Ok(FileBackedConf {
                marker: "init".to_string(),
            })
        }

        fn safe_clean(_: &str) -> orion_conf::error::OrionConfResult<()> {
            Ok(())
        }
    }

    let tmp_dir = std::env::temp_dir();
    let unique = format!(
        "wp_conf_base_try_load_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );

    let missing = tmp_dir.join(format!("{}_missing.toml", unique));
    assert!(matches!(
        FileBackedConf::try_load(missing.to_str().unwrap()),
        Ok(None)
    ));

    let valid = tmp_dir.join(format!("{}_valid.toml", unique));
    std::fs::write(&valid, "ok").unwrap();
    let loaded = FileBackedConf::try_load(valid.to_str().unwrap()).unwrap();
    assert_eq!(
        loaded,
        Some(FileBackedConf {
            marker: "ok".to_string()
        })
    );

    let invalid = tmp_dir.join(format!("{}_invalid.toml", unique));
    std::fs::write(&invalid, "boom").unwrap();
    let err = FileBackedConf::try_load(invalid.to_str().unwrap()).unwrap_err();
    assert!(err.to_string().contains("invalid content"));

    let _ = std::fs::remove_file(valid);
    let _ = std::fs::remove_file(invalid);
}

#[test]
fn test_validate_trait_default() {
    struct TestStruct;

    impl Validate for TestStruct {}

    // Default implementation should return Ok
    let result = TestStruct.validate();
    assert!(result.is_ok());
}
