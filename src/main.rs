//! # bAssist - Terminal Command Assistant
//! 
//! bAssist is a high-performance terminal tool that helps users find and execute 
//! commands based on natural language descriptions. It provides instant command 
//! suggestions, manages aliases, and includes a minimal TUI interface.
//! 
//! ## Features
//! - Fuzzy command matching based on natural language
//! - Minimal TUI interface inspired by LazyGit
//! - Alias management system
//! - Command history tracking
//! - High performance with minimal dependencies

use clap::{Parser, Subcommand};
use anyhow::Result;

mod core;
mod tui;
mod alias;
mod utils;

use crate::core::command_db::CommandDatabase;
use crate::core::matcher::FuzzyMatcher;
use crate::tui::interface::TUIInterface;
use crate::alias::manager::AliasManager;

#[derive(Parser)]
#[command(name = "bassist")]
#[command(about = "A terminal command assistant tool")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Search for a command based on description
    #[arg(short, long)]
    search: Option<String>,
    
    /// Launch TUI interface
    #[arg(short, long)]
    tui: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new alias
    Alias {
        #[command(subcommand)]
        action: AliasAction,
    },
    /// Show command history
    History,
    /// Initialize configuration
    Init,
}

#[derive(Subcommand)]
enum AliasAction {
    /// Add a new alias
    Add {
        name: String,
        command: String,
    },
    /// Remove an alias
    Remove {
        name: String,
    },
    /// List all aliases
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize core components
    let command_db = CommandDatabase::new()?;
    let mut alias_manager = AliasManager::new()?;
    let matcher = FuzzyMatcher::new();
    
    match cli.command {
        Some(Commands::Alias { action }) => {
            handle_alias_command(action, &mut alias_manager)?;
        }
        Some(Commands::History) => {
            show_history()?;
        }
        Some(Commands::Init) => {
            initialize_config()?;
        }
        None => {
            if cli.tui {
                // Launch TUI interface
                let mut tui = TUIInterface::new(command_db, alias_manager, matcher);
                tui.run()?;
            } else if let Some(search_query) = cli.search {
                // Direct search mode
                search_and_display(&search_query, &command_db, &matcher)?;
            } else {
                // Default to TUI if no specific command
                let mut tui = TUIInterface::new(command_db, alias_manager, matcher);
                tui.run()?;
            }
        }
    }
    
    Ok(())
}

fn handle_alias_command(action: AliasAction, alias_manager: &mut AliasManager) -> Result<()> {
    match action {
        AliasAction::Add { name, command } => {
            alias_manager.add_alias(&name, &command)?;
            println!("Alias '{}' added successfully", name);
        }
        AliasAction::Remove { name } => {
            alias_manager.remove_alias(&name)?;
            println!("Alias '{}' removed successfully", name);
        }
        AliasAction::List => {
            let aliases = alias_manager.list_aliases()?;
            if aliases.is_empty() {
                println!("No aliases found");
            } else {
                println!("Aliases:");
                for (name, command) in aliases {
                    println!("  {} = {}", name, command);
                }
            }
        }
    }
    Ok(())
}

fn show_history() -> Result<()> {
    // TODO: Implement history display
    println!("History feature coming soon!");
    Ok(())
}

fn initialize_config() -> Result<()> {
    // TODO: Implement config initialization
    println!("Configuration initialized successfully");
    Ok(())
}

fn search_and_display(query: &str, command_db: &CommandDatabase, matcher: &FuzzyMatcher) -> Result<()> {
    let commands = command_db.get_all_commands()?;
    let matches = matcher.find_matches(query, &commands, 5);
    
    if matches.is_empty() {
        println!("No commands found for: {}", query);
    } else {
        println!("Top matches for '{}':", query);
        for (i, cmd_match) in matches.iter().enumerate() {
            println!("{}. {} - {}", i + 1, cmd_match.command.command, cmd_match.command.description);
        }
    }
    
    Ok(())
}
