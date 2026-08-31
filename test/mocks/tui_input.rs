//! Mock implementation of `TuiInput` for testing interactive TUI logic headlessly.
//!
//! Instead of prompting on a real terminal / opening native dialogs, the mock
//! pops pre-queued responses from FIFO buffers. Each interactive read consumes
//! one queued value, which lets tests script a full menu / action sequence.

use std::cell::RefCell;
use std::collections::VecDeque;

use eck::errors::EnkryptitError;
use eck::frontend::tui::input::TuiInput;

pub struct MockTuiInput {
    selects: RefCell<VecDeque<String>>,
    texts: RefCell<VecDeque<String>>,
    counters: RefCell<VecDeque<u8>>,
    file_picks: RefCell<VecDeque<Vec<String>>>,
    folder_picks: RefCell<VecDeque<Vec<String>>>,
}

impl Default for MockTuiInput {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTuiInput {
    pub fn new() -> Self {
        Self {
            selects: RefCell::new(VecDeque::new()),
            texts: RefCell::new(VecDeque::new()),
            counters: RefCell::new(VecDeque::new()),
            file_picks: RefCell::new(VecDeque::new()),
            folder_picks: RefCell::new(VecDeque::new()),
        }
    }

    /// Append a `select` menu response (the label that will be "chosen").
    pub fn with_select(mut self, choice: &str) -> Self {
        self.selects.get_mut().push_back(choice.to_string());
        self
    }

    /// Append a `text` prompt response.
    pub fn with_text(mut self, value: &str) -> Self {
        self.texts.get_mut().push_back(value.to_string());
        self
    }

    /// Append a numeric (thread count) response.
    pub fn with_counter(mut self, value: u8) -> Self {
        self.counters.get_mut().push_back(value);
        self
    }

    /// Append a `pick_files` native-dialog result.
    pub fn with_files(mut self, paths: Vec<&str>) -> Self {
        self.file_picks
            .get_mut()
            .push_back(paths.into_iter().map(String::from).collect());
        self
    }

    /// Append a `pick_folders` native-dialog result.
    pub fn with_folders(mut self, paths: Vec<&str>) -> Self {
        self.folder_picks
            .get_mut()
            .push_back(paths.into_iter().map(String::from).collect());
        self
    }

    /// Number of `select` responses still queued (for dispatch assertions).
    pub fn pending_selects(&self) -> usize {
        self.selects.borrow().len()
    }

    /// Number of `text` responses still queued.
    pub fn pending_texts(&self) -> usize {
        self.texts.borrow().len()
    }

    /// Number of `pick_files` results still queued.
    pub fn pending_files(&self) -> usize {
        self.file_picks.borrow().len()
    }

    /// Number of `pick_folders` results still queued.
    pub fn pending_folders(&self) -> usize {
        self.folder_picks.borrow().len()
    }
}

impl TuiInput for MockTuiInput {
    fn select(&self, _message: &str, _choices: &[&str]) -> Result<String, EnkryptitError> {
        self.selects
            .borrow_mut()
            .pop_front()
            .ok_or(EnkryptitError::CommandNotFound)
    }

    fn text(&self, _message: &str, _help_message: &str) -> Result<String, EnkryptitError> {
        self.texts
            .borrow_mut()
            .pop_front()
            .ok_or(EnkryptitError::CommandNotFound)
    }

    fn custom_counter(&self, _message: &str) -> Result<u8, EnkryptitError> {
        self.counters
            .borrow_mut()
            .pop_front()
            .ok_or(EnkryptitError::CommandNotFound)
    }

    fn pick_files(&self, _title: &str) -> Vec<String> {
        self.file_picks.borrow_mut().pop_front().unwrap_or_default()
    }

    fn pick_folders(&self, _title: &str) -> Vec<String> {
        self.folder_picks
            .borrow_mut()
            .pop_front()
            .unwrap_or_default()
    }
}
