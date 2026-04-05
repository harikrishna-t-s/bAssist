//! # bAssist Library
//! 
//! This is the main library for bAssist, a high-performance terminal command 
//! assistant tool. It provides the core functionality that can be used both
//! by the CLI application and as a library for other tools.
//! 
//! ## Features
//! - Command database with fuzzy matching
//! - Alias management system
//! - History tracking
//! - TUI interface components
//! - Configuration management
//! 
//! ## Example Usage
//! 
//! ```rust
//! use bassist::core::command_db::CommandDatabase;
//! use bassist::core::matcher::FuzzyMatcher;
//! 
//! // Initialize components
//! let db = CommandDatabase::new()?;
//! let matcher = FuzzyMatcher::new();
//! 
//! // Search for commands
//! let commands = db.get_all_commands()?;
//! let matches = matcher.find_matches("git add", &commands, 5);
//! 
//! for cmd_match in matches {
//!     println!("{} - {}", cmd_match.command.command, cmd_match.command.description);
//! }
//! ```

pub mod core;
pub mod tui;
pub mod alias;
pub mod utils;

// Re-export commonly used types for convenience
pub use crate::core::command_db::{Command, CommandDatabase, CommandFlag};
pub use crate::core::matcher::{FuzzyMatcher, CommandMatch};
pub use crate::core::history::{HistoryManager, HistoryEntry, HistoryType};
pub use crate::alias::manager::AliasManager;
pub use crate::utils::config::{Config, ConfigManager};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_imports() {
        // Test that all main components can be imported
        let _ = CommandDatabase::new();
        let _ = FuzzyMatcher::new();
        let _ = AliasManager::new();
    }
}
