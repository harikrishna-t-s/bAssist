//! # Fuzzy Matcher Module
//! 
//! This module implements the fuzzy matching algorithm that powers bAssist's
//! command search functionality. It uses a combination of string similarity
//! algorithms and keyword matching to provide relevant command suggestions.
//! 
//! ## Algorithm
//! - Levenshtein distance for string similarity
//! - Keyword matching with priority scoring
//! - Category-based weighting
//! - Usage frequency influence

use crate::core::command_db::Command;
use std::cmp::Ordering;

/// Represents a matched command with its relevance score
#[derive(Debug, Clone)]
pub struct CommandMatch {
    /// The matched command
    pub command: Command,
    /// Relevance score (higher is better)
    pub score: f64,
    /// Matched keywords
    pub matched_keywords: Vec<String>,
}

/// Fuzzy matching engine
pub struct FuzzyMatcher {
    /// Weight for exact keyword matches
    keyword_weight: f64,
    /// Weight for string similarity
    similarity_weight: f64,
    /// Weight for usage frequency
    usage_weight: f64,
}

impl FuzzyMatcher {
    /// Create a new fuzzy matcher with default weights
    pub fn new() -> Self {
        Self {
            keyword_weight: 0.6,
            similarity_weight: 0.3,
            usage_weight: 0.1,
        }
    }
    
    /// Find the best matching commands for a query with context
    pub fn find_matches_with_context(&self, query: &str, commands: &[Command], limit: usize, context: &str) -> Vec<CommandMatch> {
        let mut matches: Vec<CommandMatch> = commands
            .iter()
            .map(|cmd| self.calculate_match_score_with_context(query, cmd, context))
            .filter(|match_| match_.score > 0.0)
            .collect();
        
        // Sort by score (descending)
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
        
        // Return top matches
        matches.truncate(limit);
        matches
    }
    
    /// Find the best matching commands for a query
    pub fn find_matches(&self, query: &str, commands: &[Command], limit: usize) -> Vec<CommandMatch> {
        let mut matches: Vec<CommandMatch> = commands
            .iter()
            .map(|cmd| self.calculate_match_score(query, cmd))
            .filter(|match_| match_.score > 0.0)
            .collect();
        
        // Sort by score (descending)
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
        
        // Return top matches
        matches.truncate(limit);
        matches
    }
    
    /// Calculate match score for a single command with context
    fn calculate_match_score_with_context(&self, query: &str, command: &Command, context: &str) -> CommandMatch {
        let query_lower = query.to_lowercase();
        let mut matched_keywords = Vec::new();
        
        // Calculate keyword match score
        let keyword_score = self.calculate_keyword_score(&query_lower, command, &mut matched_keywords);
        
        // Calculate string similarity score
        let similarity_score = self.calculate_similarity_score(&query_lower, command);
        
        // Calculate usage frequency score (normalized)
        let usage_score = self.calculate_usage_score(command);
        
        // Calculate context relevance score
        let context_score = self.calculate_context_score(command, context);
        
        // Combine scores with weights
        let total_score = (keyword_score * self.keyword_weight)
            + (similarity_score * self.similarity_weight)
            + (usage_score * self.usage_weight)
            + (context_score * 0.2); // Context weight
        
        CommandMatch {
            command: command.clone(),
            score: total_score,
            matched_keywords,
        }
    }
    
    /// Calculate match score for a single command
    fn calculate_match_score(&self, query: &str, command: &Command) -> CommandMatch {
        let query_lower = query.to_lowercase();
        let mut matched_keywords = Vec::new();
        
        // Calculate keyword match score
        let keyword_score = self.calculate_keyword_score(&query_lower, command, &mut matched_keywords);
        
        // Calculate string similarity score
        let similarity_score = self.calculate_similarity_score(&query_lower, command);
        
        // Calculate usage frequency score (normalized)
        let usage_score = self.calculate_usage_score(command);
        
        // Combine scores with weights
        let total_score = (keyword_score * self.keyword_weight)
            + (similarity_score * self.similarity_weight)
            + (usage_score * self.usage_weight);
        
        CommandMatch {
            command: command.clone(),
            score: total_score,
            matched_keywords,
        }
    }
    
    /// Calculate keyword matching score
    fn calculate_keyword_score(&self, query: &str, command: &Command, matched_keywords: &mut Vec<String>) -> f64 {
        let query_words: Vec<&str> = query.split_whitespace().collect();
        let mut matches = 0;
        
        for word in &query_words {
            for keyword in &command.keywords {
                if keyword.to_lowercase().contains(word) {
                    matches += 1;
                    matched_keywords.push(keyword.clone());
                    break;
                }
            }
            
            // Also check command and description
            if command.command.to_lowercase().contains(word) 
                || command.description.to_lowercase().contains(word) {
                matches += 1;
            }
        }
        
        if query_words.is_empty() {
            0.0
        } else {
            matches as f64 / query_words.len() as f64
        }
    }
    
    /// Calculate string similarity score using Levenshtein distance
    fn calculate_similarity_score(&self, query: &str, command: &Command) -> f64 {
        let command_text = format!("{} {}", command.command, command.description).to_lowercase();
        
        let distance = self.levenshtein_distance(query, &command_text);
        let max_len = query.len().max(command_text.len());
        
        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len as f64)
        }
    }
    
    /// Calculate normalized usage frequency score
    fn calculate_usage_score(&self, command: &Command) -> f64 {
        // Simple logarithmic scaling for usage frequency
        (command.usage_count as f64 + 1.0).ln() / 10.0
    }
    
    /// Calculate context relevance score
    fn calculate_context_score(&self, command: &Command, context: &str) -> f64 {
        let mut score = 0.0;
        
        // Boost score if command category matches current tool
        if command.category == context {
            score += 0.8;
        }
        
        // Simple context-based scoring
        match context {
            "kubernetes" => {
                if command.category == "kubernetes" {
                    score += 0.5;
                }
            }
            "terraform" => {
                if command.category == "terraform" {
                    score += 0.5;
                }
            }
            "docker" => {
                if command.category == "docker" {
                    score += 0.5;
                }
            }
            "git" => {
                if command.category == "git" {
                    score += 0.5;
                }
            }
            _ => {}
        }
        
        score
    }
    
    /// Calculate Levenshtein distance between two strings
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();
        
        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        // Initialize first row and column
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
                matrix[i][j] = [
                    matrix[i - 1][j] + 1,      // deletion
                    matrix[i][j - 1] + 1,      // insertion
                    matrix[i - 1][j - 1] + cost, // substitution
                ].iter().min().copied().unwrap();
            }
        }
        
        matrix[len1][len2]
    }
    
    /// Find exact matches for quick lookup
    pub fn find_exact_matches(&self, query: &str, commands: &[Command]) -> Vec<Command> {
        let query_lower = query.to_lowercase();
        commands
            .iter()
            .filter(|cmd| {
                cmd.command.to_lowercase().contains(&query_lower)
                    || cmd.description.to_lowercase().contains(&query_lower)
                    || cmd.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }
    
    /// Get suggestions based on partial input
    pub fn get_suggestions(&self, partial: &str, commands: &[Command], limit: usize) -> Vec<String> {
        let mut suggestions: Vec<String> = commands
            .iter()
            .flat_map(|cmd| {
                let mut sugs = Vec::new();
                
                // Add command suggestions
                if cmd.command.to_lowercase().starts_with(&partial.to_lowercase()) {
                    sugs.push(cmd.command.clone());
                }
                
                // Add keyword suggestions
                for keyword in &cmd.keywords {
                    if keyword.to_lowercase().starts_with(&partial.to_lowercase()) {
                        sugs.push(keyword.clone());
                    }
                }
                
                sugs
            })
            .collect();
        
        // Remove duplicates and limit
        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(limit);
        suggestions
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new()
    }
}
