//! # Configuration Module
//! 
//! This module handles configuration management for bAssist.
//! It manages user preferences, settings, and application configuration.

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::path::PathBuf;
use dirs::home_dir;
use std::fs;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Maximum number of search results
    pub max_search_results: usize,
    /// Enable history tracking
    pub enable_history: bool,
    /// Maximum history entries
    pub max_history_entries: usize,
    /// Default mode on startup
    pub default_mode: String,
    /// Enable fuzzy matching
    pub enable_fuzzy_matching: bool,
    /// Show command descriptions
    pub show_descriptions: bool,
    /// Auto-execute on single match
    pub auto_execute_single: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_search_results: 20,
            enable_history: true,
            max_history_entries: 1000,
            default_mode: "search".to_string(),
            enable_fuzzy_matching: true,
            show_descriptions: true,
            auto_execute_single: false,
        }
    }
}

/// Configuration manager
pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let mut manager = Self {
            config_path,
            config: Config::default(),
        };
        
        manager.load_config()?;
        Ok(manager)
    }
    
    /// Get the path to the configuration file
    fn get_config_path() -> Result<PathBuf> {
        let home = home_dir().context("Could not find home directory")?;
        let bassist_dir = home.join(".bassist");
        
        // Create directory if it doesn't exist
        if !bassist_dir.exists() {
            fs::create_dir_all(&bassist_dir)
                .context("Could not create .bassist directory")?;
        }
        
        Ok(bassist_dir.join("config.json"))
    }
    
    /// Load configuration from file
    fn load_config(&mut self) -> Result<()> {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)
                .context("Could not read config file")?;
            
            let config: Config = serde_json::from_str(&content)
                .context("Could not parse config file")?;
            
            self.config = config;
        }
        Ok(())
    }
    
    /// Save configuration to file
    pub fn save_config(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.config)
            .context("Could not serialize config")?;
        
        fs::write(&self.config_path, json)
            .context("Could not write config to file")?;
        
        Ok(())
    }
    
    /// Get configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    /// Update configuration
    pub fn update_config<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Config),
    {
        updater(&mut self.config);
        self.save_config()?;
        Ok(())
    }
    
    /// Reset configuration to defaults
    pub fn reset_to_defaults(&mut self) -> Result<()> {
        self.config = Config::default();
        self.save_config()?;
        Ok(())
    }
}
