//! # TUI Interface Module
//! 
//! This module implements the main terminal user interface for bAssist.
//! It provides a LazyGit-inspired interface with minimal design and
//! maximum functionality.
//! 
//! ## Features
//! - Keyboard navigation (arrows, enter, escape)
//! - Real-time search
//! - Command preview and execution
//! - Alias management interface
//! - History browsing

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use anyhow::Result;

use crate::core::command_db::{CommandDatabase, Command};
use crate::core::matcher::{FuzzyMatcher, CommandMatch};
use crate::alias::manager::AliasManager;

/// Application state
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Search,
    Browse,
    Alias,
    History,
}

/// Main TUI application
pub struct TUIInterface {
    /// Command database
    command_db: CommandDatabase,
    /// Alias manager
    alias_manager: AliasManager,
    /// Fuzzy matcher
    matcher: FuzzyMatcher,
    /// Current application state
    state: AppState,
    /// Search query
    search_query: String,
    /// Current search results
    search_results: Vec<CommandMatch>,
    /// Selected item index
    selected_index: usize,
    /// All commands for browsing
    all_commands: Vec<Command>,
    /// Should exit
    should_exit: bool,
}

impl TUIInterface {
    /// Create a new TUI interface
    pub fn new(command_db: CommandDatabase, alias_manager: AliasManager, matcher: FuzzyMatcher) -> Self {
        let all_commands = command_db.get_all_commands().unwrap_or_default();
        
        Self {
            command_db,
            alias_manager,
            matcher,
            state: AppState::Search,
            search_query: String::new(),
            search_results: Vec::new(),
            selected_index: 0,
            all_commands,
            should_exit: false,
        }
    }
    
    /// Run the TUI application
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        // Initial search
        self.update_search();
        
        // Main loop
        while !self.should_exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        
        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        
        Ok(())
    }
    
    /// Draw the UI
    fn draw(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Footer
            ])
            .split(f.size());
        
        // Draw header
        self.draw_header(f, chunks[0]);
        
        // Draw main content based on state
        match self.state {
            AppState::Search => self.draw_search(f, chunks[1]),
            AppState::Browse => self.draw_browse(f, chunks[1]),
            AppState::Alias => self.draw_alias(f, chunks[1]),
            AppState::History => self.draw_history(f, chunks[1]),
        }
        
        // Draw footer
        self.draw_footer(f, chunks[2]);
    }
    
    /// Draw header
    fn draw_header(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let header_text = match self.state {
            AppState::Search => "bAssist - Search Commands",
            AppState::Browse => "bAssist - Browse Commands",
            AppState::Alias => "bAssist - Manage Aliases",
            AppState::History => "bAssist - Command History",
        };
        
        let header = Paragraph::new(header_text)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        
        f.render_widget(header, area);
    }
    
    /// Draw search interface
    fn draw_search(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Search input
                Constraint::Min(0),     // Results
            ])
            .split(area);
        
        // Search input
        let search_text = format!("Search: {}", self.search_query);
        let search_input = Paragraph::new(search_text)
            .block(Block::default().borders(Borders::ALL).title("Query"));
        f.render_widget(search_input, chunks[0]);
        
        // Search results
        let items: Vec<ListItem> = self.search_results
            .iter()
            .enumerate()
            .map(|(i, cmd_match)| {
                let style = if i == self.selected_index {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                
                let content = format!(
                    "{} - {} (Score: {:.2})",
                    cmd_match.command.command,
                    cmd_match.command.description,
                    cmd_match.score
                );
                
                ListItem::new(content).style(style)
            })
            .collect();
        
        let results_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Results"));
        
        f.render_widget(results_list, chunks[1]);
    }
    
    /// Draw browse interface
    fn draw_browse(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let items: Vec<ListItem> = self.all_commands
            .iter()
            .enumerate()
            .map(|(i, cmd)| {
                let style = if i == self.selected_index {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                
                let content = format!(
                    "[{}] {} - {}",
                    cmd.category,
                    cmd.command,
                    cmd.description
                );
                
                ListItem::new(content).style(style)
            })
            .collect();
        
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("All Commands"));
        
        f.render_widget(list, area);
    }
    
    /// Draw alias interface
    fn draw_alias(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let aliases = self.alias_manager.list_aliases().unwrap_or_default();
        
        let items: Vec<ListItem> = aliases
            .iter()
            .enumerate()
            .map(|(i, (name, command))| {
                let style = if i == self.selected_index {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                
                let content = format!("{} = {}", name, command);
                ListItem::new(content).style(style)
            })
            .collect();
        
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Aliases"));
        
        f.render_widget(list, area);
    }
    
    /// Draw history interface
    fn draw_history(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let history_text = "History feature coming soon...";
        let paragraph = Paragraph::new(history_text)
            .block(Block::default().borders(Borders::ALL).title("History"));
        
        f.render_widget(paragraph, area);
    }
    
    /// Draw footer with key bindings
    fn draw_footer(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let help_text = match self.state {
            AppState::Search => "↑↓: Navigate | Enter: Execute | Tab: Switch Mode | Esc: Quit | Type to search",
            AppState::Browse => "↑↓: Navigate | Enter: Execute | Tab: Switch Mode | Esc: Quit",
            AppState::Alias => "↑↓: Navigate | Enter: Execute | Tab: Switch Mode | Esc: Quit | a: Add | d: Delete",
            AppState::History => "↑↓: Navigate | Tab: Switch Mode | Esc: Quit",
        };
        
        let footer = Paragraph::new(help_text)
            .style(Style::default().add_modifier(Modifier::DIM))
            .block(Block::default().borders(Borders::ALL));
        
        f.render_widget(footer, area);
    }
    
    /// Handle keyboard events
    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    self.should_exit = true;
                }
                KeyCode::Tab => {
                    self.switch_mode();
                }
                KeyCode::Up => {
                    self.move_selection(-1);
                }
                KeyCode::Down => {
                    self.move_selection(1);
                }
                KeyCode::Enter => {
                    self.execute_selected()?;
                }
                KeyCode::Char(c) => {
                    if self.state == AppState::Search {
                        self.search_query.push(c);
                        self.update_search();
                    } else if c == 'a' && self.state == AppState::Alias {
                        // TODO: Add alias
                    } else if c == 'd' && self.state == AppState::Alias {
                        // TODO: Delete alias
                    }
                }
                KeyCode::Backspace => {
                    if self.state == AppState::Search && !self.search_query.is_empty() {
                        self.search_query.pop();
                        self.update_search();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    /// Switch between different modes
    fn switch_mode(&mut self) {
        self.state = match self.state {
            AppState::Search => AppState::Browse,
            AppState::Browse => AppState::Alias,
            AppState::Alias => AppState::History,
            AppState::History => AppState::Search,
        };
        self.selected_index = 0;
    }
    
    /// Move selection up or down
    fn move_selection(&mut self, direction: isize) {
        let max_items = match self.state {
            AppState::Search => self.search_results.len(),
            AppState::Browse => self.all_commands.len(),
            AppState::Alias => self.alias_manager.list_aliases().unwrap_or_default().len(),
            AppState::History => 0,
        };
        
        if max_items == 0 {
            return;
        }
        
        let new_index = self.selected_index as isize + direction;
        if new_index >= 0 && new_index < max_items as isize {
            self.selected_index = new_index as usize;
        }
    }
    
    /// Execute selected command
    fn execute_selected(&mut self) -> Result<()> {
        match self.state {
            AppState::Search => {
                if self.selected_index < self.search_results.len() {
                    let command = self.search_results[self.selected_index].command.command.clone();
                    self.execute_command(&command)?;
                }
            }
            AppState::Browse => {
                if self.selected_index < self.all_commands.len() {
                    let command = self.all_commands[self.selected_index].command.clone();
                    self.execute_command(&command)?;
                }
            }
            AppState::Alias => {
                let aliases = self.alias_manager.list_aliases().unwrap_or_default();
                if self.selected_index < aliases.len() {
                    let command = aliases[self.selected_index].1.clone();
                    self.execute_command(&command)?;
                }
            }
            AppState::History => {
                // TODO: Execute from history
            }
        }
        Ok(())
    }
    
    /// Execute a command (copy to clipboard for now)
    fn execute_command(&mut self, command: &str) -> Result<()> {
        // For now, just print the command
        // In a real implementation, you might:
        // 1. Copy to clipboard
        // 2. Execute directly
        // 3. Return to shell
        
        println!("\nCommand to execute: {}", command);
        println!("Press Enter to continue...");
        
        // Clear screen and show command
        self.should_exit = true;
        Ok(())
    }
    
    /// Update search results
    fn update_search(&mut self) {
        if self.search_query.is_empty() {
            self.search_results.clear();
        } else {
            self.search_results = self.matcher.find_matches(
                &self.search_query,
                &self.all_commands,
                20
            );
        }
        self.selected_index = 0;
    }
}
