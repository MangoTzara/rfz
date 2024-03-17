use std::{sync::Arc, thread::available_parallelism};

use nucleo::{
    pattern::{CaseMatching, Normalization},
    Config, Nucleo,
};

pub struct SearchEngine {
    matcher: Nucleo<String>,
    pub query: String,
}

impl SearchEngine {
    pub(crate) fn injector(&self) -> nucleo::Injector<String> {
        self.matcher.injector()
    }

    /// Creates a new [`SearchEngine`].
    pub fn new(matcher: Nucleo<String>, query: String) -> Self {
        Self { matcher, query }
    }

    pub(crate) fn reparse(&mut self) {
        self.matcher.pattern.reparse(
            0,
            self.query.as_str(),
            CaseMatching::Ignore,
            Normalization::Never,
            false,
        )
    }
    #[allow(dead_code)]
    pub(crate) fn restart(&mut self, clear_snapshot: bool) {
        self.matcher.restart(clear_snapshot)
    }

    pub(crate) fn snapshot(&self) -> &nucleo::Snapshot<String> {
        self.matcher.snapshot()
    }

    pub(crate) fn tick(&mut self, arg: u64) {
        self.matcher.tick(arg);
    }

    #[allow(dead_code)]
    pub(crate) fn update_config(&mut self, config: Config) {
        self.matcher.update_config(config)
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        let num_threads = Some(available_parallelism().unwrap().get());
        Self::new(
            Nucleo::new(Config::DEFAULT, Arc::new(|| {}), num_threads, 1),
            String::new(),
        )
    }
}
