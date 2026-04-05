//! # Command Database Module
//! 
//! This module handles the storage and retrieval of command information.
//! It manages a JSON-based database of commands with their metadata,
//! including descriptions, categories, keywords, and usage statistics.
//! 
//! ## Features
//! - JSON-based persistent storage
//! - Command categorization
//! - Keyword indexing for fast search
//! - Usage statistics tracking
//! - Automatic database initialization

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::path::PathBuf;
use dirs::home_dir;
use std::fs;

/// Represents a single command in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Unique identifier for the command
    pub id: String,
    /// Command category (e.g., "git", "docker", "system")
    pub category: String,
    /// Search keywords for matching
    pub keywords: Vec<String>,
    /// The actual command to execute
    pub command: String,
    /// Human-readable description
    pub description: String,
    /// Available flags and their descriptions
    pub flags: Vec<CommandFlag>,
    /// Usage frequency counter
    pub usage_count: u64,
}

/// Represents a command flag with its description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandFlag {
    /// Flag name (e.g., "--force", "-f")
    pub flag: String,
    /// Description of what the flag does
    pub description: String,
    /// Whether the flag is required
    pub required: bool,
}

/// Main command database manager
pub struct CommandDatabase {
    /// Path to the database file
    db_path: PathBuf,
    /// In-memory cache of commands
    commands: Vec<Command>,
}

impl CommandDatabase {
    /// Create a new command database instance
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        let mut db = Self {
            db_path,
            commands: Vec::new(),
        };
        
        // Initialize database if it doesn't exist
        db.initialize_database()?;
        db.load_commands()?;
        
        Ok(db)
    }
    
    /// Get the path to the database file
    fn get_db_path() -> Result<PathBuf> {
        let home = home_dir().context("Could not find home directory")?;
        let bassist_dir = home.join(".bassist");
        
        // Create directory if it doesn't exist
        if !bassist_dir.exists() {
            fs::create_dir_all(&bassist_dir)
                .context("Could not create .bassist directory")?;
        }
        
        Ok(bassist_dir.join("commands.json"))
    }
    
    /// Initialize the database with default commands if it doesn't exist
    fn initialize_database(&self) -> Result<()> {
        if !self.db_path.exists() {
            let default_commands = Self::get_default_commands();
            let json = serde_json::to_string_pretty(&default_commands)
                .context("Could not serialize default commands")?;
            
            fs::write(&self.db_path, json)
                .context("Could not write default commands to file")?;
        }
        Ok(())
    }
    
    /// Load commands from the database file
    fn load_commands(&mut self) -> Result<()> {
        let content = fs::read_to_string(&self.db_path)
            .context("Could not read command database")?;
        
        // Parse as JSON object with commands array
        let data: serde_json::Value = serde_json::from_str(&content)
            .context("Could not parse command database")?;
        
        let commands: Vec<Command> = if let Some(commands_array) = data.get("commands").and_then(|v| v.as_array()) {
            serde_json::from_value(serde_json::Value::Array(commands_array.clone()))
                .context("Could not parse commands array")?
        } else {
            // Fallback: try parsing as direct array
            serde_json::from_str(&content)
                .context("Could not parse command database as array")?
        };
        
        self.commands = commands;
        Ok(())
    }
    
    /// Get all commands from the database
    pub fn get_all_commands(&self) -> Result<Vec<Command>> {
        Ok(self.commands.clone())
    }
    
    /// Get commands by category
    pub fn get_commands_by_category(&self, category: &str) -> Result<Vec<Command>> {
        Ok(self.commands
            .iter()
            .filter(|cmd| cmd.category == category)
            .cloned()
            .collect())
    }
    
    /// Increment usage count for a command
    pub fn increment_usage(&mut self, command_id: &str) -> Result<()> {
        if let Some(command) = self.commands.iter_mut().find(|cmd| cmd.id == command_id) {
            command.usage_count += 1;
            self.save_commands()?;
        }
        Ok(())
    }
    
    /// Save commands to the database file
    fn save_commands(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.commands)
            .context("Could not serialize commands")?;
        
        fs::write(&self.db_path, json)
            .context("Could not write commands to file")?;
        
        Ok(())
    }
    
    /// Get default commands for initialization
    fn get_default_commands() -> Vec<Command> {
        vec![
            Command {
                id: "git_add_all".to_string(),
                category: "git".to_string(),
                keywords: vec!["add".to_string(), "stage".to_string(), "all".to_string(), "files".to_string()],
                command: "git add .".to_string(),
                description: "Add all files to staging area".to_string(),
                flags: vec![
                    CommandFlag {
                        flag: "--all".to_string(),
                        description: "Also add ignored files".to_string(),
                        required: false,
                    },
                ],
                usage_count: 0,
            },
            Command {
                id: "git_commit".to_string(),
                category: "git".to_string(),
                keywords: vec!["commit".to_string(), "save".to_string(), "changes".to_string()],
                command: "git commit -m".to_string(),
                description: "Commit staged changes with message".to_string(),
                flags: vec![
                    CommandFlag {
                        flag: "-m".to_string(),
                        description: "Commit message".to_string(),
                        required: true,
                    },
                    CommandFlag {
                        flag: "--amend".to_string(),
                        description: "Amend previous commit".to_string(),
                        required: false,
                    },
                ],
                usage_count: 0,
            },
            Command {
                id: "git_push".to_string(),
                category: "git".to_string(),
                keywords: vec!["push".to_string(), "upload".to_string(), "remote".to_string()],
                command: "git push".to_string(),
                description: "Push commits to remote repository".to_string(),
                flags: vec![
                    CommandFlag {
                        flag: "--force".to_string(),
                        description: "Force push".to_string(),
                        required: false,
                    },
                    CommandFlag {
                        flag: "--set-upstream".to_string(),
                        description: "Set upstream branch".to_string(),
                        required: false,
                    },
                ],
                usage_count: 0,
            },
            Command {
                id: "docker_run".to_string(),
                category: "docker".to_string(),
                keywords: vec!["docker".to_string(), "run".to_string(), "container".to_string()],
                command: "docker run".to_string(),
                description: "Run a Docker container".to_string(),
                flags: vec![
                    CommandFlag {
                        flag: "-d".to_string(),
                        description: "Run in detached mode".to_string(),
                        required: false,
                    },
                    CommandFlag {
                        flag: "-p".to_string(),
                        description: "Port mapping".to_string(),
                        required: false,
                    },
                    CommandFlag {
                        flag: "--name".to_string(),
                        description: "Container name".to_string(),
                        required: false,
                    },
                ],
                usage_count: 0,
            },
            Command {
                id: "find_file".to_string(),
                category: "system".to_string(),
                keywords: vec!["find".to_string(), "search".to_string(), "file".to_string()],
                command: "find . -name".to_string(),
                description: "Find files by name".to_string(),
                flags: vec![
                    CommandFlag {
                        flag: "-type".to_string(),
                        description: "File type (f for file, d for directory)".to_string(),
                        required: false,
                    },
                    CommandFlag {
                        flag: "-iname".to_string(),
                        description: "Case-insensitive name search".to_string(),
                        required: false,
                    },
                ],
                usage_count: 0,
            },
            Command {
                id: "list_files".to_string(),
                category: "system".to_string(),
                keywords: vec!["list".to_string(), "files".to_string(), "directory".to_string()],
                command: "ls -la".to_string(),
                description: "List all files with details".to_string(),
                flags: vec![
                    CommandFlag {
                        flag: "-h".to_string(),
                        description: "Human readable sizes".to_string(),
                        required: false,
                    },
                    CommandFlag {
                        flag: "-r".to_string(),
                        description: "Reverse order".to_string(),
                        required: false,
                    },
                    CommandFlag {
                        flag: "-t".to_string(),
                        description: "Sort by time".to_string(),
                        required: false,
                    },
                ],
                usage_count: 0,
            },
        ]
    }
}
