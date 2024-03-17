use crate::search_engine::SearchEngine;
use nucleo::{Config, Matcher, Utf32String};
use ratatui::widgets::ListState;
use std::error;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    list_state: ListState,
    top: u32,
    matcher: SearchEngine,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    // Constructs a new instance of [`App`].
    pub fn new() -> App {
        Self {
            running: true,
            list_state: ListState::default().with_selected(Some(0)),
            matcher: SearchEngine::default(),
            top: 1000,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.matcher.tick(10);
    }

    /// Set running to false to quit the application.
    /// * `esc` - Boolean representing if the application exit shouldn't print the selected value
    pub fn quit(&mut self, esc: bool) {
        if esc {
            self.list_state.select(None);
        }
        self.running = false;
    }
    pub fn get_matched_items(&self) -> u32 {
        self.snapshot().matched_item_count()
    }
    pub fn get_total_items(&self) -> u32 {
        self.snapshot().item_count()
    }

    pub fn get_list_state(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn increment_counter(&mut self) {
        match self.list_state.selected() {
            Some(c) => {
                self.list_state.select(Some(c + 1));
                if c + 100 > self.top as usize {
                    self.top += 100;
                }
            }
            None => self.list_state.select(Some(0)),
        };
    }

    pub fn decrement_counter(&mut self) {
        self.list_state
            .select(self.list_state.selected().unwrap_or(0).checked_sub(1));
    }

    pub fn selected(&self) -> Option<&String> {
        match self.list_state.selected() {
            Some(selected) => Some(
                self.matcher
                    .snapshot()
                    .get_matched_item(selected.try_into().unwrap())
                    .unwrap()
                    .data,
            ),
            None => None,
        }
    }

    pub fn update_query(&mut self, query: char) {
        self.matcher.query.push(query);
        self.reparse();
        self.matcher.tick(10);
        self.list_state.select(Some(0));
    }

    fn reparse(&mut self) {
        self.matcher.reparse()
    }

    pub(crate) fn delete(&mut self) {
        if self.matcher.query.pop().is_some() {
            self.reparse();
            self.matcher.tick(10);
            self.list_state.select(Some(0));
        };
    }

    pub fn injector(&self) -> nucleo::Injector<String> {
        self.matcher.injector()
    }

    fn snapshot(&self) -> &nucleo::Snapshot<String> {
        self.matcher.snapshot()
    }

    pub fn get_items(&self) -> Vec<String> {
        let matched_items = match self.snapshot().matched_item_count() < self.top {
            true => self
                .snapshot()
                .matched_items(0..self.snapshot().matched_item_count()),
            false => self.snapshot().matched_items(0..self.top),
        };
        let mut res: Vec<String> = Vec::new();
        for (i, c) in matched_items.enumerate() {
            match i > self.top as usize {
                true => break,
                false => res.push(c.data.clone()),
            }
        }
        res
    }

    pub fn add_item(&mut self, to_push: String) {
        self.injector().push(to_push.clone(), |s| {
            s[0] = Utf32String::Ascii(to_push.into());
        });
    }

    pub fn get_items_with_indices(&mut self) -> Vec<(String, Vec<u32>)> {
        let mut vec: Vec<(String, Vec<u32>)> = Vec::new();

        let mut matcher = Matcher::new(Config::DEFAULT);
        let matched_items = match self.snapshot().matched_item_count() < self.top {
            true => self
                .snapshot()
                .matched_items(0..self.snapshot().matched_item_count()),
            false => self.snapshot().matched_items(0..self.top),
        };
        for (i, c) in matched_items.enumerate() {
            if i > self.top as usize {
                break;
            }
            let mut indices: Vec<u32> = Vec::new();
            self.snapshot().pattern().column_pattern(0).indices(
                c.matcher_columns[0].slice(..),
                &mut matcher,
                &mut indices,
            );
            indices.sort_unstable();
            indices.dedup();
            vec.push((c.data.to_string(), indices));
        }
        vec
    }

    pub fn get_query(&self) -> &str {
        &self.matcher.query
    }
}
