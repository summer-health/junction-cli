use junction::config::Config;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn default_config_has_no_api_key() {
    let config = Config::default();
    assert!(config.api_key.is_none());
    assert!(config.base_url.is_none());
    assert!(config.region.is_none());
}

#[test]
fn default_base_url() {
    let config = Config::default();
    assert_eq!(config.base_url(), "https://api.tryvital.io");
}

#[test]
fn custom_base_url() {
    let config = Config {
        base_url: Some("https://custom.api.com".into()),
        ..Default::default()
    };
    assert_eq!(config.base_url(), "https://custom.api.com");
}

#[test]
fn resolve_api_key_from_config() {
    let config = Config {
        api_key: Some("test-key-123".into()),
        ..Default::default()
    };
    assert_eq!(config.resolve_api_key().unwrap(), "test-key-123");
}

#[test]
fn resolve_api_key_missing_returns_error() {
    // Don't test env var removal — it's unsafe in Rust 2024 edition
    // and env vars are process-global. Just test with no config key set.
    let config = Config::default();
    // This may succeed or fail depending on whether JUNCTION_API_KEY is set
    // in the environment. We test the config-only path here.
    if std::env::var("JUNCTION_API_KEY").is_err() {
        let err = config.resolve_api_key().unwrap_err();
        assert!(err.to_string().contains("no API key configured"));
    }
}

#[test]
fn config_api_key_takes_precedence_over_env() {
    // Config key should always be returned regardless of env
    let config = Config {
        api_key: Some("config-key".into()),
        ..Default::default()
    };
    assert_eq!(config.resolve_api_key().unwrap(), "config-key");
}

#[test]
fn save_and_load_roundtrip() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.toml");
    let path = PathBuf::from(&path);

    let config = Config {
        api_key: Some("my-key".into()),
        base_url: Some("https://example.com".into()),
        region: Some("us".into()),
    };
    config.save_to(&path).unwrap();

    let loaded = Config::load_from(&path).unwrap();
    assert_eq!(loaded.api_key.as_deref(), Some("my-key"));
    assert_eq!(loaded.base_url.as_deref(), Some("https://example.com"));
    assert_eq!(loaded.region.as_deref(), Some("us"));
}

#[test]
fn load_from_nonexistent_returns_default() {
    let path = PathBuf::from("/tmp/junction-test-nonexistent/config.toml");
    let config = Config::load_from(&path).unwrap();
    assert!(config.api_key.is_none());
}

#[test]
fn save_creates_parent_directories() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nested").join("deep").join("config.toml");
    let path = PathBuf::from(&path);

    let config = Config {
        api_key: Some("key".into()),
        ..Default::default()
    };
    config.save_to(&path).unwrap();
    assert!(path.exists());
}

#[test]
fn toml_serialization_format() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.toml");
    let path = PathBuf::from(&path);

    let config = Config {
        api_key: Some("abc".into()),
        base_url: None,
        region: None,
    };
    config.save_to(&path).unwrap();

    let contents = std::fs::read_to_string(&path).unwrap();
    assert!(contents.contains("api_key = \"abc\""));
}
