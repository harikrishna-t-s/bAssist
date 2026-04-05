//! # TUI Components Module
//! 
//! This module contains reusable UI components for the bAssist TUI.
//! It provides building blocks for creating consistent and functional
//! terminal interfaces.

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// Command list component
pub struct CommandList {
    items: Vec<String>,
    selected: usize,
}

impl CommandList {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            selected: 0,
        }
    }
    
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }
    
    pub fn render(&self) -> List {
        let list_items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                
                ListItem::new(item.as_str()).style(style)
            })
            .collect();
        
        List::new(list_items)
            .block(Block::default().borders(Borders::ALL))
    }
}

/// Status bar component
pub struct StatusBar {
    text: String,
}

impl StatusBar {
    pub fn new(text: String) -> Self {
        Self { text }
    }
    
    pub fn render(&self) -> Paragraph {
        Paragraph::new(self.text.as_str())
            .style(Style::default().add_modifier(Modifier::DIM))
            .block(Block::default().borders(Borders::ALL))
    }
}

/// Help text component
pub struct HelpText {
    bindings: Vec<(String, String)>,
}

impl HelpText {
    pub fn new() -> Self {
        Self {
            bindings: vec![
                ("↑↓".to_string(), "Navigate".to_string()),
                ("Enter".to_string(), "Execute".to_string()),
                ("Tab".to_string(), "Switch Mode".to_string()),
                ("Esc".to_string(), "Quit".to_string()),
            ],
        }
    }
    
    pub fn render(&self) -> Paragraph {
        let help_text: Vec<Span> = self.bindings
            .iter()
            .flat_map(|(key, desc)| {
                vec![
                    Span::raw(format!("{}: ", key)),
                    Span::raw(format!("{}  ", desc)),
                ]
            })
            .collect();
        
        Paragraph::new(Line::from(help_text))
            .style(Style::default().add_modifier(Modifier::DIM))
    }
}
