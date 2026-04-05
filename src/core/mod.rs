//! # Core Module
//! 
//! This module contains the core functionality of bAssist, including:
//! - Command database management
//! - Fuzzy matching algorithms
//! - History tracking
//! 
//! The core module is responsible for data storage, retrieval, and the 
//! intelligent matching algorithms that power bAssist's command suggestions.

pub mod command_db;
pub mod matcher;
pub mod history;
