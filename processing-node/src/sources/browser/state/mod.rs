use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use crate::sources::Sources;

use super::settings::Settings;

pub struct State {
    settings: Settings,
    title: String,
    output_lines: Vec<String>,
    done: bool,
    current_row: usize,
    sources: Sources<File>,
}
impl State {
    pub(crate) fn new(sources: Sources<File>) -> Self {
        // let d = dirs::config_dir().unwrap();

        let mut entries = sources
            .categories()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        entries.sort();
        let settings = Settings::new();
        Self {
            settings,
            title: String::from("Smalltalk Class Browser"),
            output_lines: entries,
            done: false,
            current_row: 0,
            sources,
        }
    }

    pub(crate) fn output_lines(&self) -> &Vec<String> {
        &self.output_lines
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub(crate) fn move_current_row(&mut self, arg: i32) -> bool {
        let new_row = self.current_row as i64 + arg as i64;
        if new_row >= 0 && new_row < self.output_lines.len() as i64 {
            self.current_row = new_row as usize;
            true
        } else {
            false
        }
    }

    pub(crate) fn current_row(&self) -> usize {
        self.current_row
    }
    
    pub(crate) fn sources(&self) -> &Sources<std::fs::File> {
        &self.sources
    }
}
