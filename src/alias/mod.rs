//! # Alias Management Module
//! 
//! This module handles user-defined aliases for bAssist.
//! It provides functionality to:
//! - Create custom aliases
//! - Remove existing aliases
//! - List all aliases
//! - Persist aliases to local storage
//! 
//! Aliases are stored in a JSON file in the user's home directory
//! under ~/.bassist/aliases.json.

pub mod manager;
