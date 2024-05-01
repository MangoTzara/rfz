use std::{sync::Arc, thread::available_parallelism};

use nucleo::{
    pattern::{CaseMatching, Normalization},
    Config, Nucleo,
};

pub struct SearchEngine {
    matcher: Nucleo<String>,
}

impl SearchEngine {
    pub(crate) fn injector(&self) -> nucleo::Injector<String> {
        self.matcher.injector()
    }

    /// Creates a new [`SearchEngine`].
    pub fn new(matcher: Nucleo<String>) -> Self {
        Self { matcher }
    }

    pub(crate) fn reparse(&mut self, query: &str) {
        self.matcher
            .pattern
            .reparse(0, query, CaseMatching::Ignore, Normalization::Never, false)
    }
    #[allow(dead_code)]
    pub(crate) fn restart(&mut self, clear_snapshot: bool) {
        self.matcher.restart(clear_snapshot)
    }

    pub(crate) fn snapshot(&self) -> &nucleo::Snapshot<String> {
        self.matcher.snapshot()
    }
    #[inline]
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
        Self::new(Nucleo::new(
            Config::DEFAULT,
            Arc::new(|| {}),
            num_threads,
            1,
        ))
    }
}
