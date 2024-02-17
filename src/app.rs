use std::{env, error, ops::Add};
use tui_textarea::TextArea;

use ratatui::{text::Span, widgets::ListState};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub paths: Vec<String>,
    pub list_state: ListState,
    pub query: String,
}
impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            paths: walkdir::WalkDir::new(env::current_dir().unwrap().to_str().unwrap().to_string())
                .into_iter()
                .filter_map(|path| {
                    let dent = path.ok()?;
                    let path = dent.into_path().to_string_lossy().into_owned();
                    Some(path)
                })
                .collect(),
            list_state: ListState::default(),
            query: String::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(path: String) -> Self {
        Self {
            running: true,
            paths: walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|path| {
                    let dent = path.ok()?;
                    let path = dent.into_path().to_string_lossy().into_owned();
                    Some(path)
                })
                .collect(),
            list_state: ListState::default(),
            query: String::new(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.list_state
            .select(self.list_state.selected().unwrap_or(1).checked_sub(1));
    }

    pub fn decrement_counter(&mut self) {
        self.list_state
            .select(self.list_state.selected().unwrap_or(0).checked_add(1));
    }

    pub fn update_query(&mut self, query: char) {
        self.query.push(query);
    }

    pub(crate) fn delete(&mut self) {
        match self.query.pop() {
            Some(_) => {}
            None => {}
        };
    }
}
