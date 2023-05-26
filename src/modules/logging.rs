// Dependency Imports
use chrono::{DateTime, Utc};
use crossterm::style::Stylize;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use crate::log_tier::LogTier;
use crate::modules::module::Module;

/// The Logging Module Struct
///
/// # Parameters
/// * `data` - Vec that holds a tuple of the log messages
///
/// # Usage
/// ```
///     use crate::modules::logging::Logging;
///
///     let logging = Logging::default()
/// ```
pub struct Logging {
    pub data: Vec<(String, i16, DateTime<Utc>)>
}

/// Sets the default values for the logging module
impl Default for Logging {
    fn default() -> Self {
        Self { data: vec![] }
    }
}

/// Sets the Module trait up for logging
impl Module for Logging {
    fn get_id(&self) -> i16 {
        2
    }

    fn get_name(&self) -> String {
        "Logging".to_string()
    }

    fn get_description(&self) -> String {
        "Used to store and show logging on screen".to_string()
    }

    fn get_data(&self) -> Vec<(String, i16)> {
       Vec::new()
    }
}

impl Logging {
    /// Adds a new log message to the logs based on tier
    ///
    /// # Parameters
    /// * `description` - The Log message to be displayed
    /// * `log_type` - What tier is the log message
    pub fn log(&mut self, description: String, log_type: LogTier) {
        match log_type {
            LogTier::Info => {
                self.data.push((description, 1, Utc::now()))
            },
            LogTier::Error => {
                self.data.push((description, 2,  Utc::now()))
            },
            LogTier::Warning => {
                self.data.push((description, 3,  Utc::now()))
            },
            LogTier::Debug => {
                self.data.push((description, 4,  Utc::now()))
            }
        }
    }

    /// Gets and displays the logs messages on screen
    ///
    /// # Parameters
    /// * `frame` - Frame for drawing on the terminal
    /// * `chunks` - Layout of the frame
    pub fn get_formatted_logs<B: Backend>(&mut self, frame: &mut Frame<B>, chunks: &Vec<Rect>) {

        // Formatting all Logs to the appropriate format
        let mut logs: Vec<String> = Vec::new();
        for (desc, tier, date) in self.data.clone() {
            match tier {
                1 => {
                    logs.push(format!("{0} [{1}] {2}", date.format("%a %b %e %T %Y"),"INFO", desc.trim()));
                },
                2 => {
                    logs.push(format!("{0} [{1}] {2}", date.format("%a %b %e %T %Y"),"ERROR", desc.trim()));
                },
                3 => {
                    logs.push(format!("{0} [{1}] {2}", date.format("%a %b %e %T %Y"),"WARN", desc.trim()));
                },
                4 => {
                    logs.push(format!("{0} [{1}] {2}", date.format("%a %b %e %T %Y")   ,"DEBUG", desc.trim()));
                }
                _ => {}
            }
        }

        /// Create the Items for the list with proper styling
        let items: Vec<ListItem> = logs.iter().map( |i| {
            let mut item = ListItem::new(i.clone());
            if i.contains("ERROR") {
                item = item.style(Style::default().fg(Color::Red));
            }
            else if i.contains("WARN") {
                item = item.style(Style::default().fg(Color::Yellow));
            }
            else if i.contains("INFO") {
                item = item.style(Style::default().fg(Color::Blue));
            }
            else {
                item = item.style(Style::default().fg(Color::Green));
            }

            item
        }).collect();
        /// Create the list for the terminal
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Logging"));
        /// Render the list
        frame.render_widget(list, chunks[2]);
    }
}