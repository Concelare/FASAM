// Dependency Imports
use chrono::{DateTime, Timelike, Utc};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BarChart, Block, Borders};
use crate::modules::module::Module;
use rand::Rng;

/// The Alarm Graph Module
///
/// # Parameters
/// * `data` - Used to store the data that builds the graph
/// * `alarms_triggered` - Keeps track of how many alarms have been triggered to date
///
/// # Usage
/// ```
///     use crate::modules::alarm_graph::AlarmGraph;
///
///     let alarm_graph = AlarmGraph::default();
/// ```
pub struct AlarmGraph {
    data: Vec<(String, i16)>,
    alarms_triggered: u32
}

// Setups the default data for the AlarmGraph
impl Default for AlarmGraph {
    /// Returns an AlarmGraph
    fn default() -> Self {
        // Create obj
        let mut obj = Self {
            data: vec![],
            alarms_triggered: 0
        };

        // Used to set default data
        let mut num = 28;

        // Loops until break statement
        loop {
            // Gets a random number between 0 and 5
            let rnd = rand::thread_rng().gen_range(0..6);
            // Adds data to obj's data
            obj.data.push(((Utc::now().hour() + num).to_string(), rnd));
            // Adds amount to the alarms triggered
            obj.alarms_triggered = obj.alarms_triggered + rnd as u32;
            // When data has 29 entries break the loop
            if obj.data.iter().count() == 29 {
                break;
            }

            // Reduce num by 1
            num = num - 1;
        }

        // Return obj
        obj
    }
}

// Sets all the Module trait functions
impl Module for AlarmGraph {
    fn get_id(&self) -> i16 {
        1
    }

    fn get_name(&self) -> String {
        "Alarm Statistics".to_string()
    }

    fn get_description(&self) -> String {
        "Gets the statistics of alarms for the past 48 hours".to_string()
    }

    fn get_data(&self) -> Vec<(String, i16)> {
        return self.data.clone()
    }
}

impl AlarmGraph {
    /// Add data for the Graph
    ///
    /// # Parameters
    /// * `data` - Data to be added to the graph
    pub fn add_data(&mut self, data: (String, i16)) {
        // Removed data at 0
        self.data.remove(0);
        // Add New Data
        self.data.push(data.clone());

        let (_, num) = data.clone();
        // Set updated alarm amount
        self.alarms_triggered = self.alarms_triggered + num as u32
    }
    /// Creates the barchart and adds it to the frame
    ///
    /// # Parameters
    /// * `frame` - a Frame to add the barchart to
    /// * `chunks` - the layout of the frame
    pub fn get_barchart<B: Backend>(&self, frame: &mut Frame<B>, chunks: &Vec<Rect>) {
        // Create the barhcart vector
        let mut binding: Vec<(&str, u64)> = Vec::new();
        // Sets barchart data
        for (name, amount) in &self.data {
            binding.push((name, *amount as u64))
        }

        // Create the barchart
        let barchart = BarChart::default()
            .block(Block::default().title("Past 29 Hours Alarms").borders(Borders::ALL))
            .data(&binding)
            .bar_width(5)
            .bar_gap(3)
            .bar_style(Style::default().fg(Color::Green))
            .value_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        // Render the barchart
        frame.render_widget(barchart, chunks[1]);
    }

    /// Get the amount of alarms triggered
    pub fn get_triggered_alarms_amount(&self) -> &u32 {
        &self.alarms_triggered
    }
}