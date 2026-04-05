//! # History Module
//! 
//! This module manages command history tracking for bAssist.
//! It maintains a local history file that tracks:
//! - Commands executed through bAssist
//! - Search queries
//! - Usage patterns
//! - Timestamps
//! 
//! The history helps improve search relevance and provides quick access
//! to recently used commands.

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::path::PathBuf;
use dirs::home_dir;
use std::fs;
use chrono::{DateTime, Utc};

/// Represents a single history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Unique identifier
    pub id: String,
    /// Type of entry
    pub entry_type: HistoryType,
    /// The command or query
    pub content: String,
    /// Timestamp of the entry
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HistoryMetadata,
}

/// Types of history entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoryType {
    /// Command executed
    Command,
    /// Search query
    Search,
    /// Alias created
    AliasCreate,
    /// Alias removed
    AliasRemove,
}

/// Additional metadata for history entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMetadata {
    /// Command category (if applicable)
    pub category: Option<String>,
    /// Command ID (if applicable)
    pub command_id: Option<String>,
    /// Success status
    pub success: bool,
    /// Execution duration in milliseconds
    pub duration_ms: Option<u64>,
}

/// History manager
pub struct HistoryManager {
    /// Path to history file
    history_path: PathBuf,
    /// In-memory cache of history
    entries: Vec<HistoryEntry>,
    /// Maximum number of entries to keep
    max_entries: usize,
}

impl HistoryManager {
    /// Create a new history manager
    pub fn new() -> Result<Self> {
        let history_path = Self::get_history_path()?;
        let mut manager = Self {
            history_path,
            entries: Vec::new(),
            max_entries: 1000,
        };
        
        manager.load_history()?;
        Ok(manager)
    }
    
    /// Get the path to the history file
    fn get_history_path() -> Result<PathBuf> {
        let home = home_dir().context("Could not find home directory")?;
        let bassist_dir = home.join(".bassist");
        
        // Create directory if it doesn't exist
        if !bassist_dir.exists() {
            fs::create_dir_all(&bassist_dir)
                .context("Could not create .bassist directory")?;
        }
        
        Ok(bassist_dir.join("history.json"))
    }
    
    /// Load history from file
    fn load_history(&mut self) -> Result<()> {
        if self.history_path.exists() {
            let content = fs::read_to_string(&self.history_path)
                .context("Could not read history file")?;
            
            let entries: Vec<HistoryEntry> = serde_json::from_str(&content)
                .context("Could not parse history file")?;
            
            self.entries = entries;
        }
        Ok(())
    }
    
    /// Save history to file
    fn save_history(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)
            .context("Could not serialize history")?;
        
        fs::write(&self.history_path, json)
            .context("Could not write history to file")?;
        
        Ok(())
    }
    
    /// Add a new history entry
    pub fn add_entry(&mut self, entry_type: HistoryType, content: String, metadata: HistoryMetadata) -> Result<()> {
        use uuid::Uuid;
        
        let entry = HistoryEntry {
            id: Uuid::new_v4().to_string(),
            entry_type,
            content,
            timestamp: Utc::now(),
            metadata,
        };
        
        self.entries.push(entry);
        
        // Trim history if it exceeds max_entries
        if self.entries.len() > self.max_entries {
            self.entries.drain(0..self.entries.len() - self.max_entries);
        }
        
        self.save_history()?;
        Ok(())
    }
    
    /// Get recent history entries
    pub fn get_recent_entries(&self, limit: usize) -> Vec<HistoryEntry> {
        let start = if self.entries.len() > limit {
            self.entries.len() - limit
        } else {
            0
        };
        
        self.entries[start..].to_vec()
    }
    
    /// Get history entries by type
    pub fn get_entries_by_type(&self, entry_type: &HistoryType, limit: usize) -> Vec<HistoryEntry> {
        self.entries
            .iter()
            .filter(|entry| std::mem::discriminant(&entry.entry_type) == std::mem::discriminant(entry_type))
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
    
    /// Get command history
    pub fn get_command_history(&self, limit: usize) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| matches!(entry.entry_type, HistoryType::Command))
            .rev()
            .take(limit)
            .map(|entry| entry.content.clone())
            .collect()
    }
    
    /// Get search history
    pub fn get_search_history(&self, limit: usize) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| matches!(entry.entry_type, HistoryType::Search))
            .rev()
            .take(limit)
            .map(|entry| entry.content.clone())
            .collect()
    }
    
    /// Clear all history
    pub fn clear_history(&mut self) -> Result<()> {
        self.entries.clear();
        self.save_history()?;
        Ok(())
    }
    
    /// Get statistics about history usage
    pub fn get_statistics(&self) -> HistoryStats {
        let mut stats = HistoryStats::default();
        
        for entry in &self.entries {
            stats.total_entries += 1;
            
            match entry.entry_type {
                HistoryType::Command => stats.command_count += 1,
                HistoryType::Search => stats.search_count += 1,
                HistoryType::AliasCreate => stats.alias_create_count += 1,
                HistoryType::AliasRemove => stats.alias_remove_count += 1,
            }
            
            if let Some(duration) = entry.metadata.duration_ms {
                stats.total_duration_ms += duration;
            }
        }
        
        stats
    }
}

/// History statistics
#[derive(Debug, Default)]
pub struct HistoryStats {
    pub total_entries: usize,
    pub command_count: usize,
    pub search_count: usize,
    pub alias_create_count: usize,
    pub alias_remove_count: usize,
    pub total_duration_ms: u64,
}
