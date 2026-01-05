use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictConfig {
    pub id: String,
    pub name: String,
    pub from_lang: String,
    pub to_lang: String,
    pub path: PathBuf,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dictionaries: Vec<DictConfig>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let config = Self {
                dictionaries: Vec::new(),
            };
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config: Config =
            serde_json::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = serde_json::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Failed to get config directory")?;
        Ok(config_dir.join("tuidict").join("config.json"))
    }

    pub fn data_dir() -> Result<PathBuf> {
        let data_dir = dirs::data_dir().context("Failed to get data directory")?;
        let dict_dir = data_dir.join("tuidict").join("dictionaries");
        fs::create_dir_all(&dict_dir).context("Failed to create dictionaries directory")?;
        Ok(dict_dir)
    }

    pub fn add_dictionary(&mut self, dict: DictConfig) {
        self.dictionaries.retain(|d| d.id != dict.id);
        self.dictionaries.push(dict);
    }

    pub fn toggle_dictionary(&mut self, id: &str) -> bool {
        if let Some(dict) = self.dictionaries.iter_mut().find(|d| d.id == id) {
            dict.active = !dict.active;
            true
        } else {
            false
        }
    }

    pub fn get_active_dictionaries(&self) -> Vec<&DictConfig> {
        self.dictionaries.iter().filter(|d| d.active).collect()
    }

    pub fn remove_dictionary(&mut self, id: &str) -> Option<DictConfig> {
        if let Some(idx) = self.dictionaries.iter().position(|d| d.id == id) {
            Some(self.dictionaries.remove(idx))
        } else {
            None
        }
    }
}
