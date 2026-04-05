//! # Alias Manager Module
//! 
//! This module implements the alias management functionality for bAssist.
//! It handles the creation, removal, listing, and persistence of user-defined
//! command aliases.

use anyhow::{Result, Context};
use std::path::PathBuf;
use dirs::home_dir;
use std::fs;
use std::collections::HashMap;

/// Alias manager for handling user-defined command aliases
pub struct AliasManager {
    /// Path to aliases file
    aliases_path: PathBuf,
    /// In-memory cache of aliases
    aliases: HashMap<String, String>,
}

impl AliasManager {
    /// Create a new alias manager
    pub fn new() -> Result<Self> {
        let aliases_path = Self::get_aliases_path()?;
        let mut manager = Self {
            aliases_path,
            aliases: HashMap::new(),
        };
        
        manager.load_aliases()?;
        Ok(manager)
    }
    
    /// Get the path to the aliases file
    fn get_aliases_path() -> Result<PathBuf> {
        let home = home_dir().context("Could not find home directory")?;
        let bassist_dir = home.join(".bassist");
        
        // Create directory if it doesn't exist
        if !bassist_dir.exists() {
            fs::create_dir_all(&bassist_dir)
                .context("Could not create .bassist directory")?;
        }
        
        Ok(bassist_dir.join("aliases.json"))
    }
    
    /// Load aliases from file
    fn load_aliases(&mut self) -> Result<()> {
        if self.aliases_path.exists() {
            let content = fs::read_to_string(&self.aliases_path)
                .context("Could not read aliases file")?;
            
            let aliases: HashMap<String, String> = serde_json::from_str(&content)
                .context("Could not parse aliases file")?;
            
            self.aliases = aliases;
        }
        Ok(())
    }
    
    /// Save aliases to file
    fn save_aliases(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.aliases)
            .context("Could not serialize aliases")?;
        
        fs::write(&self.aliases_path, json)
            .context("Could not write aliases to file")?;
        
        Ok(())
    }
    
    /// Add a new alias
    pub fn add_alias(&mut self, name: &str, command: &str) -> Result<()> {
        // Validate alias name
        if name.is_empty() {
            return Err(anyhow::anyhow!("Alias name cannot be empty"));
        }
        
        if command.is_empty() {
            return Err(anyhow::anyhow!("Alias command cannot be empty"));
        }
        
        // Check if alias already exists
        if self.aliases.contains_key(name) {
            return Err(anyhow::anyhow!("Alias '{}' already exists", name));
        }
        
        // Add the alias
        self.aliases.insert(name.to_string(), command.to_string());
        self.save_aliases()?;
        
        Ok(())
    }
    
    /// Remove an alias
    pub fn remove_alias(&mut self, name: &str) -> Result<()> {
        if !self.aliases.contains_key(name) {
            return Err(anyhow::anyhow!("Alias '{}' does not exist", name));
        }
        
        self.aliases.remove(name);
        self.save_aliases()?;
        
        Ok(())
    }
    
    /// Update an existing alias
    pub fn update_alias(&mut self, name: &str, command: &str) -> Result<()> {
        if !self.aliases.contains_key(name) {
            return Err(anyhow::anyhow!("Alias '{}' does not exist", name));
        }
        
        if command.is_empty() {
            return Err(anyhow::anyhow!("Alias command cannot be empty"));
        }
        
        self.aliases.insert(name.to_string(), command.to_string());
        self.save_aliases()?;
        
        Ok(())
    }
    
    /// Get an alias by name
    pub fn get_alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }
    
    /// List all aliases
    pub fn list_aliases(&self) -> Result<Vec<(String, String)>> {
        let mut aliases: Vec<(String, String)> = self.aliases
            .iter()
            .map(|(name, command)| (name.clone(), command.clone()))
            .collect();
        
        // Sort by alias name
        aliases.sort_by(|a, b| a.0.cmp(&b.0));
        
        Ok(aliases)
    }
    
    /// Check if an alias exists
    pub fn alias_exists(&self, name: &str) -> bool {
        self.aliases.contains_key(name)
    }
    
    /// Get the number of aliases
    pub fn alias_count(&self) -> usize {
        self.aliases.len()
    }
    
    /// Clear all aliases
    pub fn clear_aliases(&mut self) -> Result<()> {
        self.aliases.clear();
        self.save_aliases()?;
        Ok(())
    }
    
    /// Import aliases from a file
    pub fn import_aliases(&mut self, file_path: &str) -> Result<usize> {
        let content = fs::read_to_string(file_path)
            .context("Could not read import file")?;
        
        let import_aliases: HashMap<String, String> = serde_json::from_str(&content)
            .context("Could not parse import file")?;
        
        let mut imported_count = 0;
        for (name, command) in import_aliases {
            if !self.aliases.contains_key(&name) {
                self.aliases.insert(name, command);
                imported_count += 1;
            }
        }
        
        if imported_count > 0 {
            self.save_aliases()?;
        }
        
        Ok(imported_count)
    }
    
    /// Export aliases to a file
    pub fn export_aliases(&self, file_path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.aliases)
            .context("Could not serialize aliases for export")?;
        
        fs::write(file_path, json)
            .context("Could not write aliases to export file")?;
        
        Ok(())
    }
    
    /// Validate alias name
    pub fn validate_alias_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Alias name cannot be empty"));
        }
        
        if name.len() > 50 {
            return Err(anyhow::anyhow!("Alias name too long (max 50 characters)"));
        }
        
        // Check for invalid characters
        if name.contains(' ') || name.contains('\t') || name.contains('\n') {
            return Err(anyhow::anyhow!("Alias name cannot contain whitespace"));
        }
        
        // Check if it's a reserved keyword
        let reserved_keywords = vec![
            "add", "remove", "list", "help", "quit", "exit", "search", "history"
        ];
        
        if reserved_keywords.contains(&name.to_lowercase().as_str()) {
            return Err(anyhow::anyhow!("'{}' is a reserved keyword", name));
        }
        
        Ok(())
    }
    
    /// Validate alias command
    pub fn validate_alias_command(command: &str) -> Result<()> {
        if command.is_empty() {
            return Err(anyhow::anyhow!("Alias command cannot be empty"));
        }
        
        if command.len() > 1000 {
            return Err(anyhow::anyhow!("Alias command too long (max 1000 characters)"));
        }
        
        Ok(())
    }
}

/// Default aliases that can be loaded on first run
impl AliasManager {
    /// Get default aliases
    pub fn get_default_aliases() -> HashMap<String, String> {
        let mut aliases = HashMap::new();
        
        // Common Git aliases
        aliases.insert("gst".to_string(), "git status".to_string());
        aliases.insert("gco".to_string(), "git checkout".to_string());
        aliases.insert("gcm".to_string(), "git commit -m".to_string());
        aliases.insert("gp".to_string(), "git push".to_string());
        aliases.insert("gl".to_string(), "git pull".to_string());
        aliases.insert("ga".to_string(), "git add".to_string());
        aliases.insert("gd".to_string(), "git diff".to_string());
        
        // Common system aliases
        aliases.insert("ll".to_string(), "ls -la".to_string());
        aliases.insert("la".to_string(), "ls -A".to_string());
        aliases.insert("l".to_string(), "ls -CF".to_string());
        aliases.insert("rmf".to_string(), "rm -rf".to_string());
        
        // Navigation aliases
        aliases.insert("..".to_string(), "cd ..".to_string());
        aliases.insert("...".to_string(), "cd ../..".to_string());
        aliases.insert("....".to_string(), "cd ../../..".to_string());
        
        aliases
    }
    
    /// Load default aliases if none exist
    pub fn load_defaults_if_empty(&mut self) -> Result<()> {
        if self.aliases.is_empty() {
            self.aliases = Self::get_default_aliases();
            self.save_aliases()?;
        }
        Ok(())
    }
}
