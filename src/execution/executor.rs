//! # Command Executor Module
//! 
//! This module handles safe command execution with proper
//! error handling and output management.

use anyhow::{Result, Context};
use std::process::{Command, Stdio};
use std::io::{self, Write};
use shell_words::split;

/// Command executor
pub struct CommandExecutor {
    interactive: bool,
}

impl CommandExecutor {
    /// Create a new command executor
    pub fn new() -> Self {
        Self {
            interactive: true,
        }
    }
    
    /// Execute a command safely
    pub fn execute_command(&self, command_str: &str) -> Result<()> {
        // Parse command safely
        let args = split(command_str)
            .context("Failed to parse command")?;
        
        if args.is_empty() {
            return Err(anyhow::anyhow!("Empty command"));
        }
        
        let (program, program_args) = args.split_at(1);
        let program = &program[0];
        
        println!("Executing: {}", command_str);
        println!("Press Ctrl+C to cancel...");
        
        // For interactive execution, run directly
        if self.interactive {
            let mut cmd = Command::new(program);
            cmd.args(program_args);
            
            // Inherit stdin/stdout/stderr for interactive experience
            cmd.stdin(Stdio::inherit());
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
            
            let status = cmd.status()
                .context("Failed to execute command")?;
            
            if !status.success() {
                let code = status.code().unwrap_or(-1);
                return Err(anyhow::anyhow!("Command failed with exit code: {}", code));
            }
        } else {
            // For non-interactive, capture output
            let output = Command::new(program)
                .args(program_args)
                .output()
                .context("Failed to execute command")?;
            
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
            
            if !output.status.success() {
                let code = output.status.code().unwrap_or(-1);
                return Err(anyhow::anyhow!("Command failed with exit code: {}", code));
            }
        }
        
        Ok(())
    }
    
    /// Execute command and capture output
    pub fn execute_command_capture(&self, command_str: &str) -> Result<(String, String)> {
        let args = split(command_str)
            .context("Failed to parse command")?;
        
        if args.is_empty() {
            return Err(anyhow::anyhow!("Empty command"));
        }
        
        let (program, program_args) = args.split_at(1);
        let program = &program[0];
        
        let output = Command::new(program)
            .args(program_args)
            .output()
            .context("Failed to execute command")?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            return Err(anyhow::anyhow!("Command failed with exit code: {}: {}", code, stderr));
        }
        
        Ok((stdout, stderr))
    }
    
    /// Check if command would succeed (dry run)
    pub fn dry_run_command(&self, command_str: &str) -> Result<bool> {
        let args = split(command_str)
            .context("Failed to parse command")?;
        
        if args.is_empty() {
            return Ok(false);
        }
        
        let (program, program_args) = args.split_at(1);
        let program = &program[0];
        
        // Check if command exists
        if which::which(program).is_err() {
            return Ok(false);
        }
        
        // For commands that support --dry-run, use that
        if program == "kubectl" || program == "terraform" || program == "helm" {
            let mut dry_run_args = program_args.to_vec();
            dry_run_args.insert(0, program.to_string());
            
            if program == "kubectl" {
                dry_run_args.push("--dry-run=client".to_string());
            } else if program == "terraform" {
                // terraform plan is the dry run equivalent
                if dry_run_args.contains(&"apply".to_string()) {
                    dry_run_args = vec!["terraform".to_string(), "plan".to_string()];
                }
            }
            
            let output = Command::new(&args[0])
                .args(&dry_run_args[1..])
                .output();
            
            match output {
                Ok(result) => Ok(result.status.success()),
                Err(_) => Ok(false),
            }
        } else {
            // For other commands, just check if they exist
            Ok(true)
        }
    }
    
    /// Set interactive mode
    pub fn set_interactive(&mut self, interactive: bool) {
        self.interactive = interactive;
    }
    
    /// Get command suggestion based on partial input
    pub fn get_command_suggestion(&self, partial: &str) -> Option<String> {
        // This is a simple implementation - could be enhanced with ML
        if partial.starts_with("git") {
            if partial.contains("commit") {
                return Some("git commit -m \"\"".to_string());
            } else if partial.contains("push") {
                return Some("git push".to_string());
            } else if partial.contains("add") {
                return Some("git add .".to_string());
            }
        } else if partial.starts_with("kubectl") {
            if partial.contains("get") {
                return Some("kubectl get pods".to_string());
            } else if partial.contains("apply") {
                return Some("kubectl apply -f".to_string());
            }
        } else if partial.starts_with("docker") {
            if partial.contains("run") {
                return Some("docker run -d".to_string());
            } else if partial.contains("ps") {
                return Some("docker ps -a".to_string());
            }
        }
        
        None
    }
}
