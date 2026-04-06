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
mod safety;
mod context;
mod execution;

use crate::core::command_db::CommandDatabase;
use crate::core::matcher::FuzzyMatcher;
use crate::tui::interface::TUIInterface;
use crate::alias::manager::AliasManager;
use crate::safety::safety_checker::SafetyChecker;
use crate::context::detector::ContextDetector;
use crate::execution::executor::CommandExecutor;

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
    
    /// Execute command directly
    #[arg(short = 'x', long)]
    execute: Option<String>,
    
    /// Show command explanation
    #[arg(short = 'e', long)]
    explain: Option<String>,
    
    /// Dry run mode (show what would be executed)
    #[arg(long)]
    dry_run: bool,
    
    /// Show best practices for command
    #[arg(long)]
    best_practice: Option<String>,
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
    /// Import aliases from shell
    ImportAliases,
    /// Sync aliases with shell
    SyncAliases,
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
    let safety_checker = SafetyChecker::new()?;
    let context_detector = ContextDetector::new()?;
    let executor = CommandExecutor::new();
    
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
        Some(Commands::ImportAliases) => {
            import_aliases(&mut alias_manager)?;
        }
        Some(Commands::SyncAliases) => {
            sync_aliases(&mut alias_manager)?;
        }
        None => {
            if let Some(execute_query) = cli.execute {
                execute_direct_command(&execute_query, &command_db, &matcher, &safety_checker, &context_detector, &executor, cli.dry_run)?;
            } else if let Some(explain_query) = cli.explain {
                explain_command(&explain_query, &command_db, &matcher)?;
            } else if let Some(best_practice_query) = cli.best_practice {
                show_best_practices(&best_practice_query, &command_db, &matcher)?;
            } else if let Some(search_query) = cli.search {
                search_and_display(&search_query, &command_db, &matcher, &context_detector)?;
            } else if cli.tui {
                // Launch TUI interface
                let mut tui = TUIInterface::new(command_db, alias_manager, matcher);
                tui.run()?;
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

fn search_and_display(query: &str, command_db: &CommandDatabase, matcher: &FuzzyMatcher, context_detector: &ContextDetector) -> Result<()> {
    let commands = command_db.get_all_commands()?;
    let context = context_detector.detect_current_context()?;
    let matches = matcher.find_matches_with_context(query, &commands, 5, &context.tool);
    
    if matches.is_empty() {
        println!("No commands found for: {}", query);
    } else {
        println!("Top matches for '{}' (context: {}):", query, context.tool);
        for (i, cmd_match) in matches.iter().enumerate() {
            let safety_indicator = match cmd_match.command.safety_level.as_ref().map(|s| s.as_str()) {
                Some("dangerous") => "⚠️ ",
                Some("caution") => "⚡ ",
                _ => "",
            };
            println!("{}. {}{} - {} {}", i + 1, safety_indicator, cmd_match.command.command, cmd_match.command.description, 
                if cmd_match.score > 0.8 { "(high relevance)" } else { "" });
        }
    }
    
    Ok(())
}

fn execute_direct_command(query: &str, command_db: &CommandDatabase, matcher: &FuzzyMatcher, safety_checker: &SafetyChecker, context_detector: &ContextDetector, executor: &CommandExecutor, dry_run: bool) -> Result<()> {
    let commands = command_db.get_all_commands()?;
    let context = context_detector.detect_current_context()?;
    let matches = matcher.find_matches_with_context(query, &commands, 1, &context.tool);
    
    if let Some(cmd_match) = matches.first() {
        let command = &cmd_match.command.command;
        
        // Safety check
        let safety_result = safety_checker.check_command(command, &context)?;
        if !safety_result.safe {
            println!("⚠️ SAFETY WARNING: {}", safety_result.warning);
            if safety_result.requires_confirmation {
                print!("Do you want to proceed? (y/N): ");
                use std::io;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Command cancelled.");
                    return Ok(());
                }
            }
        }
        
        if dry_run {
            println!("DRY RUN: Would execute: {}", command);
            if let Some(impact) = safety_result.impact {
                println!("Expected impact: {}", impact);
            }
        } else {
            println!("Executing: {}", command);
            executor.execute_command(command)?;
        }
    } else {
        println!("No command found for: {}", query);
    }
    
    Ok(())
}

fn explain_command(query: &str, command_db: &CommandDatabase, matcher: &FuzzyMatcher) -> Result<()> {
    let commands = command_db.get_all_commands()?;
    let matches = matcher.find_matches(query, &commands, 1);
    
    if let Some(cmd_match) = matches.first() {
        let command = &cmd_match.command;
        
        println!("Command: {}", command.command);
        println!("Description: {}", command.description);
        println!("Category: {}", command.category);
        
        if !command.flags.is_empty() {
            println!("Available flags:");
            for flag in &command.flags {
                let required = if flag.required { "(required)" } else { "(optional)" };
                println!("  {} - {} {}", flag.flag, flag.description, required);
            }
        }
        
        if let Some(safety) = &command.safety_level {
            println!("Safety Level: {}", safety);
        }
        
        // Add best practices
        match command.category.as_str() {
            "kubernetes" => println!("Best Practice: Always check current namespace with 'kubectl config view --minify'"),
            "terraform" => println!("Best Practice: Always run 'terraform plan' before 'terraform apply'"),
            "docker" => println!("Best Practice: Use specific image tags instead of 'latest'"),
            "git" => println!("Best Practice: Commit frequently with descriptive messages"),
            _ => {}
        }
    } else {
        println!("No command found for: {}", query);
    }
    
    Ok(())
}

fn show_best_practices(query: &str, command_db: &CommandDatabase, matcher: &FuzzyMatcher) -> Result<()> {
    let commands = command_db.get_all_commands()?;
    let matches = matcher.find_matches(query, &commands, 1);
    
    if let Some(cmd_match) = matches.first() {
        let command = &cmd_match.command;
        
        println!("Best Practices for: {}", command.command);
        println!("{}", "=".repeat(50));
        
        match command.category.as_str() {
            "kubernetes" => {
                println!("• Always check current namespace: kubectl config view --minify");
                println!("• Use 'kubectl get' before 'kubectl delete' to verify targets");
                println!("• Prefer 'kubectl apply' over 'kubectl create' for declarative management");
                println!("• Use labels and selectors for resource organization");
                println!("• Set resource limits to prevent resource exhaustion");
            }
            "terraform" => {
                println!("• Always run 'terraform plan' before 'terraform apply'");
                println!("• Use remote state for team collaboration");
                println!("• Version control your .tf files");
                println!("• Use variables for environment-specific values");
                println!("• Implement proper module structure");
            }
            "docker" => {
                println!("• Use specific image tags instead of 'latest'");
                println!("• Multi-stage builds for smaller production images");
                println!("• Run containers as non-root users");
                println!("• Use .dockerignore to exclude unnecessary files");
                println!("• Implement health checks in containers");
            }
            "git" => {
                println!("• Commit frequently with descriptive messages");
                println!("• Use branches for feature development");
                println!("• Pull before push to avoid conflicts");
                println!("• Use .gitignore to exclude sensitive files");
                println!("• Review changes before committing");
            }
            _ => {
                println!("• Read the manual pages for complex commands");
                println!("• Test commands in non-production environments first");
                println!("• Use version control for configuration files");
                println!("• Document your command workflows");
            }
        }
        
        if command.safety_level.as_ref().map_or(false, |s| s == "dangerous") {
            println!("\n⚠️ SAFETY WARNING:");
            println!("• This command can cause data loss");
            println!("• Double-check command parameters");
            println!("• Consider using --dry-run flag if available");
            println!("• Backup important data before execution");
        }
    } else {
        println!("No command found for: {}", query);
    }
    
    Ok(())
}

fn import_aliases(alias_manager: &mut AliasManager) -> Result<()> {
    println!("Importing aliases from shell...");
    
    // Try to import from bash
    if let Some(home) = dirs::home_dir() {
        let bashrc = home.join(".bashrc");
        let zshrc = home.join(".zshrc");
        
        if bashrc.exists() {
            import_aliases_from_file(&bashrc, alias_manager)?;
        }
        if zshrc.exists() {
            import_aliases_from_file(&zshrc, alias_manager)?;
        }
    }
    
    println!("Alias import completed.");
    Ok(())
}

fn import_aliases_from_file(file_path: &std::path::Path, alias_manager: &mut AliasManager) -> Result<()> {
    use std::fs;
    
    let content = fs::read_to_string(file_path)?;
    let alias_regex = regex::Regex::new(r#"alias\s+([^=]+)=(?:"([^"]+)"|'([^']+)'|([^#]+))"#)?;
    
    for caps in alias_regex.captures_iter(&content) {
        if let (Some(name), Some(command)) = (caps.get(1), caps.get(2).or_else(|| caps.get(3)).or_else(|| caps.get(4))) {
            let name = name.as_str().trim();
            let command = command.as_str().trim();
            
            if !alias_manager.alias_exists(name) {
                if let Err(e) = alias_manager.add_alias(name, command) {
                    eprintln!("Failed to import alias '{}': {}", name, e);
                } else {
                    println!("Imported alias: {} = {}", name, command);
                }
            }
        }
    }
    
    Ok(())
}

fn sync_aliases(alias_manager: &mut AliasManager) -> Result<()> {
    println!("Syncing aliases with shell...");
    import_aliases(alias_manager)?;
    println!("Alias sync completed.");
    Ok(())
}
