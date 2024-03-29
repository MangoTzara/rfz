use futures::future::PollFn;

use indexmap::map::IndexMap;
use nucleo::pattern::CaseMatching::Ignore;
use nucleo::pattern::Normalization::Never;
use nucleo::{Config, Matcher, Nucleo, Utf32String};
use ratatui::widgets::ListState;
use std::default;
use std::thread::available_parallelism;
use std::{error, sync::Arc};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub list_state: ListState,
    pub query: String,
    top: u32,
    matcher: Nucleo<String>,
}

impl App {
    pub fn default() -> App {
        let matcher: Nucleo<String> = Nucleo::new(
            Config::DEFAULT,
            Arc::new(|| {}),
            Some(available_parallelism().unwrap().get()),
            1,
        );

        Self {
            running: true,
            list_state: ListState::default().with_selected(Some(0)),
            query: String::new(),
            matcher,
            top: 1000,
        }
    }

    /// Constructs a new instance of [`App`].
    pub fn new(path: &[String]) -> Self {
        let mut matcher: Nucleo<String> = Nucleo::new(
            Config::DEFAULT,
            Arc::new(|| {}),
            Some(available_parallelism().unwrap().get()),
            1,
        );

        path.iter().for_each(|c| {
            matcher.injector().push(c.clone(), |s| {
                s[0] = Utf32String::Ascii(c.to_string().into());
            });
        });

        matcher.tick(10);

        Self {
            running: true,
            list_state: ListState::default().with_selected(Some(0)),
            query: String::new(),
            matcher,
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

    pub fn update_query(&mut self, query: char) {
        self.query.push(query);
        self.reparse();
        self.matcher.tick(10);
        self.list_state.select(Some(0));
    }

    fn reparse(&mut self) {
        self.matcher
            .pattern
            .reparse(0, self.query.as_str(), Ignore, Never, false);
    }

    pub(crate) fn delete(&mut self) {
        if self.query.pop().is_some() {
            self.reparse();
            self.matcher.tick(10);
            self.list_state.select(Some(0));
        };
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

    pub fn get_items_with_indices(&mut self) -> IndexMap<String, Vec<u32>> {
        let mut indexmap: IndexMap<String, Vec<u32>> = IndexMap::new();

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
            indexmap.insert(c.data.to_string(), indices);
        }
        indexmap
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }
}

#[cfg(test)]
mod tests {

    use super::App;

    #[test]
    fn get_items_no_search() {
        let path = vec!["asd".to_string(), "dqasd".to_string(), "adq".to_string()];
        let sut: App = App::new(&path);
        sut.get_items()
            .iter()
            .for_each(|c| assert!(path.contains(c)));
    }

    #[test]
    fn get_items_search_search() {
        let path = vec!["asd".to_string(), "dqasd".to_string(), "adq".to_string()];
        let mut sut: App = App::new(&path);
        sut.update_query('l');
        assert!(sut.get_items().is_empty());
    }

    #[test]
    fn get_indices() {
        let path = vec!["asd".to_string(), "dqasd".to_string(), "adq".to_string()];
        let mut sut: App = App::new(&path.clone());
        sut.update_query('a');
        sut.update_query('s');
        let res = sut.get_items_with_indices();
        let vec: Vec<u32> = vec![0, 1];
        assert_eq!(res.get(&"asd".to_string()).unwrap(), &vec);
    }
    #[test]
    fn empty_search_indices() {
        let path = vec!["asd".to_string(), "dqasd".to_string(), "adq".to_string()];
        let mut sut: App = App::new(&path);
        let res = sut.get_items_with_indices();
        let vec: Vec<u32> = Vec::new();
        assert_eq!(res.keys().map(|c| c.to_string()).collect::<Vec<_>>(), path);
        assert_eq!(res.get(&"asd".to_string()).unwrap(), &vec);
    }
}
