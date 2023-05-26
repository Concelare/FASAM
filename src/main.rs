// Dependency Imports
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use std::time::Duration;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use crate::ui::UI;

// Modules
mod ui;
mod modules;
mod log_tier;


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    // sets the terminal into raw mode, disabling input, enter is no longer process, CTRL+C is disabled
    enable_raw_mode()?;
    // Get output method
    let mut stdout = io::stdout();
    // Set output method, connects to crossterm screen, and starts capturing the mouse
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // creates the UI called app and runs it
    let mut app = UI::default();
    let res = app.run_app(&mut terminal, Duration::from_millis(500));

    // disables raw mode resetting the terminal back into default usage and how it normal runs
    disable_raw_mode()?;
    // Disables mouse tracking and disconnects from the crossterm screen
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    // Starts showing the cursor
    terminal.show_cursor()?;

    // Catches all Errors
    if let Err(err) = res {
        println!("{:?}", err)
    }

    // Ends the program with an Ok status
    Ok(())
}
