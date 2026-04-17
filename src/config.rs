use serde::Deserialize;
use toml;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub signaling_server: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            signaling_server: Some("ws://localhost:25134".to_string()),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_str = match std::fs::read_to_string("config.toml") {
        Ok(config_str) => config_str,
        Err(_) => {
            println!("config.toml not found, using default configuration");
            return Ok(Config::default());
        }
    };
    match toml::from_str(&config_str) {
        Ok(config) => Ok(config),
        Err(e) => {
            println!("Failed to parse config.toml: {}, using default configuration", e);
            Ok(Config::default())
        }
    }
}