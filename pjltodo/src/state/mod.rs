use std::{fs, io, path::PathBuf};

use crate::settings::Settings;

pub struct State {
    settings: Settings,
    title: String,
    output_lines: Vec<String>,
    done: bool,
    current_row: usize,
}
impl State {
    pub(crate) fn new() -> Self {
        // let d = dirs::config_dir().unwrap();
        let d = PathBuf::from("/mnt/pjl");
        let mut entries = fs::read_dir(&d)
            .unwrap()
            .map(|res| res.map(|e| e.path().as_mut_os_str().to_str().unwrap().to_string()))
            .collect::<Result<Vec<String>, io::Error>>()
            .unwrap();

        // The order in which `read_dir` returns entries is not guaranteed. If reproducible
        // ordering is required the entries should be explicitly sorted.

        entries.sort();

        let settings = Settings::new();
        Self {
            settings,
            title: d.as_os_str().to_str().unwrap().to_string(),
            output_lines: entries,
            done: false,
            current_row: 0,
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
}
