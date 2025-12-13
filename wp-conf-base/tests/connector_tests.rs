use wp_conf_base::connector::ConnectorKindAdapter;
use wp_model_core::model::fmt_def::TextFmt;

struct TestAdapter {
    kind_name: &'static str,
}

impl ConnectorKindAdapter for TestAdapter {
    fn kind(&self) -> &'static str {
        self.kind_name
    }

    fn defaults(&self) -> toml::value::Table {
        let mut table = toml::value::Table::new();
        if self.kind_name == "mysql" {
            table.insert("host".to_string(), toml::Value::String("localhost".to_string()));
            table.insert("port".to_string(), toml::Value::Integer(3306));
            table.insert("username".to_string(), toml::Value::String("root".to_string()));
        } else if self.kind_name == "file" {
            table.insert("path".to_string(), toml::Value::String("/var/log/app.log".to_string()));
            table.insert("rotation".to_string(), toml::Value::String("daily".to_string()));
        }
        table
    }

    fn url_to_params(&self, url: &str) -> anyhow::Result<toml::value::Table> {
        if self.kind_name == "mysql" {
            if let Some(url) = url.strip_prefix("mysql://") {
                let mut table = toml::value::Table::new();
                // Simple URL parsing for demonstration
                if url.contains('@') {
                    let parts: Vec<&str> = url.split('@').collect();
                    if parts.len() == 2 {
                        let auth = parts[0];
                        let host_db = parts[1];

                        if let Some((username, password)) = auth.split_once(':') {
                            table.insert("username".to_string(), toml::Value::String(username.to_string()));
                            table.insert("password".to_string(), toml::Value::String(password.to_string()));
                        }

                        if let Some((host, db)) = host_db.split_once('/') {
                            if let Some((host, port)) = host.split_once(':') {
                                table.insert("host".to_string(), toml::Value::String(host.to_string()));
                                table.insert("port".to_string(), toml::Value::Integer(port.parse().unwrap_or(3306)));
                            } else {
                                table.insert("host".to_string(), toml::Value::String(host.to_string()));
                            }
                            table.insert("database".to_string(), toml::Value::String(db.to_string()));
                        }
                    }
                }
                Ok(table)
            } else {
                Err(anyhow::anyhow!("Invalid MySQL URL format"))
            }
        } else {
            Ok(toml::value::Table::new())
        }
    }

    fn default_fmt(&self) -> Option<TextFmt> {
        if self.kind_name == "file" {
            Some(TextFmt::Json)
        } else {
            None
        }
    }
}

#[test]
fn test_connector_kind() {
    let mysql_adapter = TestAdapter { kind_name: "mysql" };
    assert_eq!(mysql_adapter.kind(), "mysql");

    let file_adapter = TestAdapter { kind_name: "file" };
    assert_eq!(file_adapter.kind(), "file");
}

#[test]
fn test_connector_defaults() {
    let mysql_adapter = TestAdapter { kind_name: "mysql" };
    let defaults = mysql_adapter.defaults();

    assert_eq!(defaults.get("host").unwrap().as_str(), Some("localhost"));
    assert_eq!(defaults.get("port").unwrap().as_integer(), Some(3306));
    assert_eq!(defaults.get("username").unwrap().as_str(), Some("root"));

    let file_adapter = TestAdapter { kind_name: "file" };
    let defaults = file_adapter.defaults();

    assert_eq!(defaults.get("path").unwrap().as_str(), Some("/var/log/app.log"));
    assert_eq!(defaults.get("rotation").unwrap().as_str(), Some("daily"));
}

#[test]
fn test_connector_url_parsing() {
    let mysql_adapter = TestAdapter { kind_name: "mysql" };

    // Test valid MySQL URL
    let params = mysql_adapter.url_to_params("mysql://user:pass@localhost:3306/mydb").unwrap();
    assert_eq!(params.get("username").unwrap().as_str(), Some("user"));
    assert_eq!(params.get("password").unwrap().as_str(), Some("pass"));
    assert_eq!(params.get("host").unwrap().as_str(), Some("localhost"));
    assert_eq!(params.get("port").unwrap().as_integer(), Some(3306));
    assert_eq!(params.get("database").unwrap().as_str(), Some("mydb"));

    // Test invalid URL
    let result = mysql_adapter.url_to_params("http://invalid");
    assert!(result.is_err());
}

#[test]
fn test_connector_default_fmt() {
    let mysql_adapter = TestAdapter { kind_name: "mysql" };
    assert!(mysql_adapter.default_fmt().is_none());

    let file_adapter = TestAdapter { kind_name: "file" };
    assert!(file_adapter.default_fmt().is_some());
    assert_eq!(file_adapter.default_fmt().unwrap(), TextFmt::Json);
}

// Test using trait object
#[test]
fn test_connector_as_trait_object() {
    let adapters: Vec<Box<dyn ConnectorKindAdapter>> = vec![
        Box::new(TestAdapter { kind_name: "mysql" }),
        Box::new(TestAdapter { kind_name: "file" }),
    ];

    for adapter in adapters {
        let kind = adapter.kind();
        let defaults = adapter.defaults();
        assert!(!defaults.is_empty(), "Adapter {} should have defaults", kind);
    }
}