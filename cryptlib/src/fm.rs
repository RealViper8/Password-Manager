#![allow(unused)]
use std::{borrow::BorrowMut, cell::RefCell, fmt::Display};

use walkdir::{DirEntry, WalkDir};

pub struct FileManager {
    cd: String,
    entry: RefCell<Vec<Option<walkdir::DirEntry>>>,
}

impl Default for FileManager {
    fn default() -> Self {
        FileManager {
            cd: String::from("."),
            entry: RefCell::new(Vec::new()),
        }
    }
}

impl FileManager {
    pub fn is_hidden(&self, entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    pub fn search(&self) {
        let entries: Vec<walkdir::DirEntry> = WalkDir::new(&self.cd)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| !self.is_hidden(e))
            .filter_map(|rs| rs.ok())
            .collect();

        for entry in entries {
            self.entry.borrow_mut().push(Some(entry));
        }
    }

    pub fn set_cd(&mut self, cd: &str) {
        self.cd = String::from(cd);
    }
}

impl Display for FileManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entry_iter = RefCell::clone(&self.entry);

        let mut tmp_string = String::new();

        let mut binding = entry_iter.borrow_mut();
        let mut iterator = binding.iter().peekable();
        while let Some(entry) = iterator.next() {
            if entry.is_some() {
                tmp_string += &entry.as_ref().unwrap().path().display().to_string();
            }
            if iterator.peek().is_some() {
                tmp_string += "\n";
            }
        }

        write!(f, "{}", tmp_string)
    }
}
