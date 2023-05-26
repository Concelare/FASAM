// Dependency Imports
use std::{
    io,
    time::{Duration, Instant},
};
use std::sync::Arc;
use chrono::{DateTime, Timelike, Utc};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use tui::{backend::Backend, Frame, Terminal};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BarChart, Block, Borders, Paragraph};
use crate::log_tier::LogTier;
use crate::modules::alarm_graph::AlarmGraph;
use crate::modules::logging::Logging;
use crate::modules::module::Module;

/// The User Interface for the FASAM Software
///
/// # Parameters
/// * `alarm` - Alarm Graph Module
/// * `logging` - Logging Module
/// * `last_hour` - Used to track alarms hour by hour
/// * `alarms_triggered` - used to track alarms triggered in the current run
/// * `last_triggered` - used to see when the alarm was last triggered
pub struct UI {
    alarm: AlarmGraph,
    logging: Logging,
    last_hour: u32,
    alarms_triggered: i16,
    last_triggered: DateTime<Utc>
}

/// Sets up the Default trait for the UI Struct
impl Default for UI {
    /// Returns the default settings for the UI Struct
    fn default() -> Self {
        Self { alarm: AlarmGraph::default(), logging: Logging::default(), last_hour: Utc::now().hour(), alarms_triggered: 0, last_triggered: Utc::now() }
    }
}

impl UI {
    /// Used to setup and run the app
    ///
    /// # Parameters
    /// * `terminal` - Terminal Backend that is being used
    /// * `tick_rate` - How often used the user interface update
    pub fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        self.logging.log("Starting the system".to_string(), LogTier::Info);
        self.logging.log("Starting Logging Module".to_string(), LogTier::Debug);
        self.logging.log("Starting Alarm Graph Module".to_string(), LogTier::Debug);
        self.logging.log("Successfully Started Logging Module".to_string(), LogTier::Info);
        self.logging.log("Successfully Started Alarm Graph Module".to_string(), LogTier::Info);



        // Loops until told to break so that the program does not randomly close
        loop {
            // Draws the user interface on the terminal
            terminal.draw(|f| self.ui(f))?;

            // timeout is used to track the tick rate if there is no tick rate then it sets to 250ms
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_millis(250));

            // Checks if a user has input a value
            if event::poll(timeout)? {
                // Gets the key that was pressed
                if let Event::Key(key) = event::read()? {
                    // Matches the key to the appropriate char
                    match key.code {
                        // Exits the program
                        KeyCode::Char('q') => return Ok(()),
                        // Triggers the alarm
                        KeyCode::Char('t') => {
                            self.logging.log("Alarm Has Been Triggered!".to_string(), LogTier::Warning);
                            self.last_triggered = Utc::now();
                            self.alarms_triggered = self.alarms_triggered + 1 as i16;
                        },
                        // Resets the alarm
                        KeyCode::Char('r') => {
                            self.logging.log("Alarm Has Been Disabled!".to_string(), LogTier::Info);
                        }
                        _ => {}
                    }
                }
            }
            // Resets the last tick
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    /// Used to draw the UI onto the terminal
    ///
    /// # Parameters
    /// * `f` - The frame that used to build the UI
    fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        // The default block settings with title and border used to separate sections
        let block = Block::default().title("FASAM").borders(Borders::ALL);
        // How the layout should be divided up and in what direction
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(35),
                    Constraint::Percentage(35),
                    Constraint::Percentage(15),
                ]
                    .as_ref(),
            )
            .split(f.size());

        // The first top paragraph
        let paragraph = Paragraph::new("Welcome to the Fire and Security Alarm Monitoring System. Please use the following key binds:\n Q - Quit\n R - Reset The Alarms\n T - Trigger The Alarm\n")
            .style(Style::default()).alignment(Alignment::Left).block(block.clone());
        // Tells the frame to render it and where
        f.render_widget(paragraph, chunks[0]);

        // update alarms if the hour has changed before rendering them
        if self.last_hour != Utc::now().hour() {
            self.alarm.add_data((Utc::now().hour().to_string(), self.alarms_triggered));
            self.alarms_triggered = 0;
            self.last_hour = Utc::now().hour();
        }

        // Loads the Barchart module on screen
        self.alarm.get_barchart(f, &chunks);
        // Loads the Logging module on screen
        self.logging.get_formatted_logs(f, &chunks);

        // Sets the footer paragraph with all stats
        let para = Paragraph::new(format!("Stats:\n Last Alarm Triggered: {0}\n Alarms Recorded To Date: {1}\n Alarm in This Run: {2}", self.last_triggered.format("%a %b %e %T %Y"), self.alarm.get_triggered_alarms_amount(), self.alarms_triggered))
            .style(Style::default()).alignment(Alignment::Left).block(block.clone());
        // Renders the footer
        f.render_widget(para, chunks[3]);
    }
}
