use crate::{query::Query, search_engine::SearchEngine};
use crossterm::event::{KeyCode, KeyEvent};
use nucleo::{Config, Matcher, Utf32String};
use ratatui::{
    style::Style,
    widgets::{ListState, Widget},
};
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
    text_area: Query,
}

impl Default for App {
    fn default() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
            top: 1000,
            text_area: Query::default(),
            matcher: SearchEngine::default(),
            running: true,
        }
    }
}

impl App {
    // Constructs a new instance of [`App`].
    pub fn new(list_state: ListState, top: u32, matcher: SearchEngine, style: Style) -> App {
        Self {
            running: true,
            list_state,
            top,
            matcher,
            text_area: Query::new(style),
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

    pub fn paste(&mut self, to_paste: &str) {
        self.text_area.push_str(to_paste);
        self.reparse();
        self.matcher.tick(10);
        self.list_state.select(Some(0));
    }

    pub fn update_query(&mut self, query: KeyEvent) {
        let KeyCode::Char(e) = query.code else {
            return;
        };
        self.text_area.push(e);
        self.reparse();
        self.matcher.tick(10);
        self.list_state.select(Some(0));
    }

    fn reparse(&mut self) {
        self.matcher.reparse(self.text_area.text());
    }

    pub(crate) fn delete(&mut self) {
        if self.text_area.pop() {
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
        for c in matched_items {
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

    pub(crate) fn get_state_area(&mut self) -> impl Widget + '_ {
        &self.text_area
    }
}
