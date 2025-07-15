//! Configuration loader for monitoring agent.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub services: Vec<String>,
    pub http_ports: HashMap<String, u16>, // port name -> port number
    pub app_port: Option<u16>,
}

impl Config {
    /// Loads config from config.toml in the current directory.
    pub fn load() -> Self {
        let content = fs::read_to_string("config.toml")
            .expect("Failed to read config.toml");
        toml::from_str(&content)
            .expect("Failed to parse config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parse() {
        let toml = r#"
            app_port = 1234
            services = ["sshd"]
            [http_ports]
            web = 80
            api = 8080
        "#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.app_port, Some(1234));
        assert_eq!(config.services, vec!["sshd"]);
        assert_eq!(config.http_ports["web"], 80);
    }
}
