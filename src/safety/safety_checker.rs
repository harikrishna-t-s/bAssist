//! # Safety Checker Module
//! 
//! This module implements safety checking for commands to prevent
//! accidental damage in production environments.

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use regex::Regex;
use std::collections::HashMap;

use crate::context::detector::ContextInfo;

/// Safety level for commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyLevel {
    Safe,
    Caution,
    Dangerous,
}

/// Safety check result
#[derive(Debug, Clone)]
pub struct SafetyResult {
    pub safe: bool,
    pub warning: String,
    pub requires_confirmation: bool,
    pub impact: Option<String>,
    pub suggestions: Vec<String>,
}

/// Safety checker for commands
pub struct SafetyChecker {
    dangerous_patterns: Vec<Regex>,
    caution_patterns: Vec<Regex>,
    environment_rules: HashMap<String, Vec<Regex>>,
}

impl SafetyChecker {
    /// Create a new safety checker
    pub fn new() -> Result<Self> {
        let mut checker = Self {
            dangerous_patterns: Vec::new(),
            caution_patterns: Vec::new(),
            environment_rules: HashMap::new(),
        };
        
        checker.initialize_patterns()?;
        Ok(checker)
    }
    
    /// Initialize dangerous and caution patterns
    fn initialize_patterns(&mut self) -> Result<()> {
        // Dangerous patterns - data loss or production impact
        let dangerous_patterns = vec![
            r"rm\s+-rf\s+/",           // rm -rf / (system destruction)
            r"rm\s+-rf\s+\.",          // rm -rf . (current directory)
            r"rm\s+-rf\s+\.\.",        // rm -rf .. (parent directory)
            r"rm\s+-rf\s+.*\*",        // rm -rf with wildcard
            r"dd\s+if=/dev/zero",      // dd with zero input
            r":()>.*\|\|.*",            // Fork bomb
            r"chmod\s+-R\s+777",       // chmod 777 recursively
            r"chown\s+-R\s+.*\s+/",    // chown recursively on root
            r"mkfs\.",                 // Filesystem formatting
            r"fdisk\s+",               // Disk partitioning
            r"wipefs\s+",              // Filesystem wiping
            r"shred\s+",               // Secure file deletion
            r"terraform\s+destroy",    // Terraform destroy
            r"kubectl\s+delete\s+namespace", // Delete namespace
            r"kubectl\s+drain",        // Drain node
            r"kubectl\s+delete\s+deployment\s+--all", // Delete all deployments
            r"docker\s+system\s+prune\s+-a", // Docker system prune all
            r"docker\s+rm\s+-f\s+\$\(docker\s+ps\s+-aq\)", // Remove all containers
            r"docker\s+rmi\s+-f\s+\$\(docker\s+images\s+-aq\)", // Remove all images
        ];
        
        for pattern in dangerous_patterns {
            self.dangerous_patterns.push(Regex::new(pattern)?);
        }
        
        // Caution patterns - potentially problematic
        let caution_patterns = vec![
            r"rm\s+-rf",               // rm -rf (any use)
            r"rm\s+-r",                // rm -r (recursive)
            r"mv\s+.*\s+/",            // Move to root directory
            r"cp\s+.*\s+/",            // Copy to root directory
            r"chmod\s+-R",             // Recursive chmod
            r"chown\s+-R",             // Recursive chown
            r"kill\s+-9",              // Force kill
            r"killall\s+-9",           // Killall force
            r"pkill\s+-9",             // Pkill force
            r"systemctl\s+stop",       // Stop system service
            r"service\s+\w+\s+stop",   // Stop service
            r"shutdown",               // System shutdown
            r"reboot",                 // System reboot
            r"halt",                   // System halt
            r"poweroff",               // Power off
            r"terraform\s+apply",      // Terraform apply
            r"kubectl\s+delete",       // Any kubectl delete
            r"kubectl\s+scale.*--replicas=0", // Scale to zero
            r"docker\s+rm",            // Remove container
            r"docker\s+rmi",           // Remove image
            r"docker\s+stop",         // Stop container
            r"docker\s+kill",         // Kill container
            r"git\s+reset\s+--hard",  // Git hard reset
            r"git\s+clean\s+-fd",     // Git clean force
            r"git\s+branch\s+-D",     // Delete branch force
        ];
        
        for pattern in caution_patterns {
            self.caution_patterns.push(Regex::new(pattern)?);
        }
        
        // Environment-specific rules
        self.environment_rules.insert("production".to_string(), vec![
            Regex::new(r"kubectl\s+delete")?,
            Regex::new(r"terraform\s+apply")?,
            Regex::new(r"docker\s+system\s+prune")?,
            Regex::new(r"docker\s+rmi")?,
            Regex::new(r"rm\s+-rf")?,
        ]);
        
        self.environment_rules.insert("kubernetes".to_string(), vec![
            Regex::new(r"kubectl\s+delete")?,
            Regex::new(r"kubectl\s+drain")?,
            Regex::new(r"kubectl\s+cordon")?,
            Regex::new(r"kubectl\s+scale.*--replicas=0")?,
        ]);
        
        Ok(())
    }
    
    /// Check if a command is safe
    pub fn check_command(&self, command: &str, context: &ContextInfo) -> Result<SafetyResult> {
        let mut safe = true;
        let mut warning = String::new();
        let mut requires_confirmation = false;
        let mut impact = None;
        let mut suggestions = Vec::new();
        
        // Check dangerous patterns
        for pattern in &self.dangerous_patterns {
            if pattern.is_match(command) {
                safe = false;
                warning = "DANGEROUS COMMAND DETECTED".to_string();
                requires_confirmation = true;
                impact = Some("This command can cause significant data loss or system damage".to_string());
                suggestions.push("Consider using --dry-run flag if available".to_string());
                suggestions.push("Backup important data before execution".to_string());
                suggestions.push("Test in non-production environment first".to_string());
                break;
            }
        }
        
        // Check caution patterns if not already dangerous
        if safe {
            for pattern in &self.caution_patterns {
                if pattern.is_match(command) {
                    safe = false;
                    warning = "CAUTION: This command may have unintended consequences".to_string();
                    requires_confirmation = true;
                    impact = Some("This command may affect system state or data".to_string());
                    suggestions.push("Double-check command parameters".to_string());
                    suggestions.push("Verify target directory/namespace".to_string());
                    break;
                }
            }
        }
        
        // Environment-specific checks
        if let Some(rules) = self.environment_rules.get(&context.environment) {
            for pattern in rules {
                if pattern.is_match(command) {
                    if safe {
                        safe = false;
                        warning = format!("CAUTION: This command affects {} environment", context.environment);
                        requires_confirmation = true;
                    }
                    impact = Some(format!("This will affect {} environment", context.environment));
                    suggestions.push("Verify you're in the correct environment".to_string());
                    suggestions.push("Consider using namespace/cluster flags".to_string());
                    break;
                }
            }
        }
        
        // Kubernetes-specific checks
        if context.tool == "kubernetes" {
            if command.contains("delete") && !command.contains("--dry-run") {
                suggestions.push("Consider using --dry-run=client to preview".to_string());
                suggestions.push("Use 'kubectl get' first to verify targets".to_string());
            }
        }
        
        // Terraform-specific checks
        if context.tool == "terraform" {
            if command.contains("apply") && !command.contains("plan") {
                suggestions.push("Always run 'terraform plan' first".to_string());
                suggestions.push("Review the plan before applying".to_string());
            }
        }
        
        // Docker-specific checks
        if context.tool == "docker" {
            if command.contains("rmi") && !command.contains("--force") {
                suggestions.push("Check if containers are using these images".to_string());
            }
        }
        
        Ok(SafetyResult {
            safe,
            warning,
            requires_confirmation,
            impact,
            suggestions,
        })
    }
    
    /// Get safety level for a command
    pub fn get_safety_level(&self, command: &str, context: &ContextInfo) -> SafetyLevel {
        // Check dangerous patterns first
        for pattern in &self.dangerous_patterns {
            if pattern.is_match(command) {
                return SafetyLevel::Dangerous;
            }
        }
        
        // Check caution patterns
        for pattern in &self.caution_patterns {
            if pattern.is_match(command) {
                return SafetyLevel::Caution;
            }
        }
        
        // Environment-specific checks
        if let Some(rules) = self.environment_rules.get(&context.environment) {
            for pattern in rules {
                if pattern.is_match(command) {
                    return SafetyLevel::Caution;
                }
            }
        }
        
        SafetyLevel::Safe
    }
    
    /// Add custom dangerous pattern
    pub fn add_dangerous_pattern(&mut self, pattern: &str) -> Result<()> {
        self.dangerous_patterns.push(Regex::new(pattern)?);
        Ok(())
    }
    
    /// Add custom caution pattern
    pub fn add_caution_pattern(&mut self, pattern: &str) -> Result<()> {
        self.caution_patterns.push(Regex::new(pattern)?);
        Ok(())
    }
}
