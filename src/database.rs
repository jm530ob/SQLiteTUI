use std::{
    fmt::Debug,
    fs::File,
    io::{self, Error, ErrorKind},
};

use rusqlite::{types, Connection};

use crate::app::{AppState, ViewState};

pub struct Cursor {
    pub x: usize,
    pub y: usize,
    // pub current: (usize, usize),
}

pub struct Db {
    pub db_name: Option<String>,
    pub table_name: Option<String>,
    pub records: Vec<types::Value>,
}

impl Db {
    pub fn new() -> rusqlite::Result<Self> {
        Ok(Self {
            db_name: None,
            table_name: None,
            records: vec![],
        })
    }

    pub fn open_db_if_exists(&self) -> Result<(), io::Error> {
        let name =
            Self::ensure_correct_path(self.db_name.clone().expect("Cannot open nameless DB file"));

        let result = File::open(name.trim());
        match result {
            Ok(_) => return Ok(()),

            Err(err) => return Err(err),
        };
    }

    pub fn create_db(&self) -> Result<(), io::Error> {
        let name = Self::ensure_correct_path(self.db_name.clone().unwrap_or("default".to_owned()));

        match File::create(name.trim()) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }

    pub fn add_record(&mut self, sql_type: types::Value) {
        self.records.push(sql_type);
    }

    pub fn add_record_list(&mut self, list: Vec<types::Value>) {
        for sql_type in list {
            self.add_record(sql_type);
        }
    }

    pub fn select_table(&self) {}

    pub fn ensure_correct_path(mut name: String) -> String {
        if !name.ends_with(".db") {
            name.push_str(".db");
        }
        name
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    #[test]
    fn create_db_file_from_path() {
        let path = "test.db";
        super::Db::new()
            .unwrap()
            .create_db()
            .expect("Failed to create a new DB file with name: {}");

        assert!(Path::new(path).exists());

        remove_file(path).expect("Failed to remove DB file");
    }
}
