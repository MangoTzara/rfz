use nucleo::{Config, Nucleo, Utf32String};
use std::{error, sync::Arc};

use ratatui::widgets::ListState;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub list_state: ListState,
    pub query: String,
    matcher: Nucleo<String>,
    path: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(path: String) -> Self {
        Self {
            path,
            running: true,
            list_state: ListState::default(),
            query: String::new(),
            matcher: Nucleo::new(Config::DEFAULT, Arc::new(|| {}), Some(4), 1),
        }
    }

    pub fn start(&mut self) {
        jwalk::WalkDir::new(&self.path)
            .into_iter()
            .for_each(|path| {
                match path {
                    Ok(p) => {
                        self.matcher
                            .injector()
                            .push(p.path().to_string_lossy().to_string(), |s| {
                                s[0] = Utf32String::Ascii(p.path().to_string_lossy().into());
                            });
                    }
                    Err(_) => {}
                };
            });
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.matcher.tick(10);
    }

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
        self.matcher.pattern.reparse(
            0,
            self.query.as_str(),
            nucleo::pattern::CaseMatching::Ignore,
            nucleo::pattern::Normalization::Never,
            true,
        );
        self.matcher.tick(10);
    }

    pub(crate) fn delete(&mut self) {
        match self.query.pop() {
            Some(_) => {}
            None => {}
        };
        self.matcher.pattern.reparse(
            0,
            self.query.as_str(),
            nucleo::pattern::CaseMatching::Ignore,
            nucleo::pattern::Normalization::Never,
            true,
        );
        self.matcher.tick(10);
    }

    pub fn injector(&self) -> nucleo::Injector<String> {
        self.matcher.injector()
    }

    pub fn restart(&mut self, clear_snapshot: bool) {
        self.matcher.restart(clear_snapshot)
    }

    pub fn snapshot(&self) -> &nucleo::Snapshot<String> {
        self.matcher.snapshot()
    }

    pub fn update_config(&mut self, config: Config) {
        self.matcher.update_config(config)
    }
}
