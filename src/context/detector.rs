//! # Context Detector Module
//! 
//! This module detects the current context to provide better
//! command suggestions and safety checks.

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::env;
use std::path::Path;
use std::fs;

/// Context information about the current environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextInfo {
    pub environment: String,
    pub tool: String,
    pub project_type: Option<String>,
    pub current_directory: String,
    pub cloud_provider: Option<String>,
    pub cluster_context: Option<String>,
    pub namespace: Option<String>,
}

/// Context detector
pub struct ContextDetector {
    current_dir: String,
}

impl ContextDetector {
    /// Create a new context detector
    pub fn new() -> Result<Self> {
        let current_dir = env::current_dir()
            .context("Could not get current directory")?
            .to_string_lossy()
            .to_string();
        
        Ok(Self {
            current_dir,
        })
    }
    
    /// Detect current context
    pub fn detect_current_context(&self) -> Result<ContextInfo> {
        let environment = self.detect_environment()?;
        let tool = self.detect_primary_tool();
        let project_type = self.detect_project_type();
        let cloud_provider = self.detect_cloud_provider();
        let (cluster_context, namespace) = self.detect_kubernetes_context();
        
        Ok(ContextInfo {
            environment,
            tool,
            project_type,
            current_directory: self.current_dir.clone(),
            cloud_provider,
            cluster_context,
            namespace,
        })
    }
    
    /// Detect environment type
    fn detect_environment(&self) -> Result<String> {
        // Check environment variables
        if let Ok(env) = env::var("ENVIRONMENT") {
            return Ok(env.to_lowercase());
        }
        
        if let Ok(env) = env::var("ENV") {
            return Ok(env.to_lowercase());
        }
        
        if let Ok(env) = env::var("NODE_ENV") {
            return Ok(env.to_lowercase());
        }
        
        // Check Kubernetes context
        if let Ok(kubeconfig) = env::var("KUBECONFIG") {
            if kubeconfig.contains("production") || kubeconfig.contains("prod") {
                return Ok("production".to_string());
            }
        }
        
        // Check directory naming
        let dir_lower = self.current_dir.to_lowercase();
        if dir_lower.contains("production") || dir_lower.contains("prod") {
            return Ok("production".to_string());
        }
        if dir_lower.contains("staging") || dir_lower.contains("stage") {
            return Ok("staging".to_string());
        }
        if dir_lower.contains("development") || dir_lower.contains("dev") {
            return Ok("development".to_string());
        }
        
        // Check Terraform workspace
        if self.command_exists("terraform") {
            if let Ok(output) = std::process::Command::new("terraform")
                .args(&["workspace", "show"])
                .output() 
            {
                if output.status.success() {
                    let workspace = String::from_utf8_lossy(&output.stdout);
                    let workspace = workspace.trim();
                    if workspace != "default" {
                        return Ok(workspace.to_lowercase());
                    }
                }
            }
        }
        
        Ok("development".to_string()) // Default
    }
    
    /// Detect primary tool based on directory and files
    fn detect_primary_tool(&self) -> String {
        let path = Path::new(&self.current_dir);
        
        // Check for Kubernetes manifests
        if path.join("k8s").exists() || 
           path.join("kubernetes").exists() ||
           path.join("deployments").exists() ||
           self.has_files_with_extension(&["yaml", "yml"]) && self.file_contains_k8s() {
            return "kubernetes".to_string();
        }
        
        // Check for Terraform
        if path.join("main.tf").exists() || 
           path.join("terraform.tf").exists() ||
           self.has_files_with_extension(&["tf"]) {
            return "terraform".to_string();
        }
        
        // Check for Docker
        if path.join("Dockerfile").exists() || 
           path.join("docker-compose.yml").exists() ||
           path.join("docker-compose.yaml").exists() {
            return "docker".to_string();
        }
        
        // Check for Git repository
        if path.join(".git").exists() {
            return "git".to_string();
        }
        
        // Check for AWS CDK
        if path.join("cdk.json").exists() ||
           path.join(".cdk").exists() {
            return "aws-cdk".to_string();
        }
        
        // Check for Ansible
        if path.join("playbook.yml").exists() ||
           path.join("playbook.yaml").exists() ||
           path.join("ansible.cfg").exists() {
            return "ansible".to_string();
        }
        
        "system".to_string()
    }
    
    /// Detect project type
    fn detect_project_type(&self) -> Option<String> {
        let path = Path::new(&self.current_dir);
        
        if path.join("package.json").exists() {
            return Some("nodejs".to_string());
        }
        
        if path.join("requirements.txt").exists() || 
           path.join("pyproject.toml").exists() ||
           path.join("setup.py").exists() {
            return Some("python".to_string());
        }
        
        if path.join("Cargo.toml").exists() {
            return Some("rust".to_string());
        }
        
        if path.join("go.mod").exists() {
            return Some("go".to_string());
        }
        
        if path.join("pom.xml").exists() || 
           path.join("build.gradle").exists() {
            return Some("java".to_string());
        }
        
        None
    }
    
    /// Detect cloud provider
    fn detect_cloud_provider(&self) -> Option<String> {
        // Check AWS
        if env::var("AWS_ACCESS_KEY_ID").is_ok() ||
           env::var("AWS_PROFILE").is_ok() ||
           Path::new(&self.current_dir).join(".aws").exists() {
            return Some("aws".to_string());
        }
        
        // Check Azure
        if env::var("AZURE_SUBSCRIPTION_ID").is_ok() ||
           Path::new(&self.current_dir).join("azure.json").exists() {
            return Some("azure".to_string());
        }
        
        // Check Google Cloud
        if env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() ||
           env::var("GCLOUD_PROJECT").is_ok() ||
           Path::new(&self.current_dir).join("gcloud.json").exists() {
            return Some("gcp".to_string());
        }
        
        None
    }
    
    /// Detect Kubernetes context and namespace
    fn detect_kubernetes_context(&self) -> (Option<String>, Option<String>) {
        if !self.command_exists("kubectl") {
            return (None, None);
        }
        
        let mut context = None;
        let mut namespace = None;
        
        // Get current context
        if let Ok(output) = std::process::Command::new("kubectl")
            .args(&["config", "current-context"])
            .output() {
            if output.status.success() {
                let ctx = String::from_utf8_lossy(&output.stdout);
                context = Some(ctx.trim().to_string());
            }
        }
        
        // Get current namespace
        if let Ok(output) = std::process::Command::new("kubectl")
            .args(&["config", "view", "--minify", "--output", "jsonpath={..namespace}"])
            .output() {
            if output.status.success() {
                let ns = String::from_utf8_lossy(&output.stdout);
                let ns = ns.trim();
                if !ns.is_empty() {
                    namespace = Some(ns.to_string());
                } else {
                    namespace = Some("default".to_string());
                }
            }
        }
        
        (context, namespace)
    }
    
    /// Check if a command exists
    fn command_exists(&self, command: &str) -> bool {
        which::which(command).is_ok()
    }
    
    /// Check if directory has files with specific extensions
    fn has_files_with_extension(&self, extensions: &[&str]) -> bool {
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                if let Some(path) = entry.path().extension() {
                    if let Some(ext) = path.to_str() {
                        if extensions.contains(&ext) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    /// Check if any YAML files contain Kubernetes resources
    fn file_contains_k8s(&self) -> bool {
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if ext_str == "yaml" || ext_str == "yml" {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if content.contains("kind:") && 
                                   (content.contains("Deployment") || 
                                    content.contains("Service") || 
                                    content.contains("ConfigMap") ||
                                    content.contains("Secret")) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }
}
