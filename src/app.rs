// src/app.rs

use crate::ui;
use crate::bridge::cli;
use ratatui::prelude::*;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub struct App {
    #[allow(dead_code)]
    pub topics: Vec<String>,
    #[allow(dead_code)]
    pub selected_index: usize,
    pub logs: Vec<String>,
    pub exit: bool,
    pub building: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            topics: Vec::new(),
            selected_index: 0,
            logs: vec!["LazyROS started".to_string(), "Press 'b' to build and install package".to_string()],
            exit: false,
            building: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui::render(f, self))?;
            
            if self.exit {
                break;
            }

            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) -> io::Result<()> {
        match key {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            KeyCode::Char('b') => {
                self.build();
            }
            _ => {}
        }
        Ok(())
    }

    fn build(&mut self) {
        if self.building {
            return;
        }
        self.building = true;
        self.logs.push("Building package with colcon...".to_string());
        
        match cli::run_colcon_build() {
            Ok(output) => {
                // Split output into lines and add them
                let lines: Vec<&str> = output.lines().collect();
                for line in lines {
                    if !line.trim().is_empty() {
                        self.logs.push(line.to_string());
                    }
                }
                self.logs.push("✓ Build successful!".to_string());
            }
            Err(e) => {
                // Split error into lines for better readability
                let lines: Vec<&str> = e.lines().collect();
                for line in lines {
                    if !line.trim().is_empty() {
                        self.logs.push(line.to_string());
                    }
                }
                self.logs.push("✗ Build failed!".to_string());
            }
        }
        
        // Keep logs manageable
        if self.logs.len() > 100 {
            self.logs.drain(0..self.logs.len() - 100);
        }
        
        self.building = false;
    }
}