use std::{
    fmt::Debug,
    fs::File,
    io::{self, Error, ErrorKind},
};

use rusqlite::{types, Connection, ToSql};

use crate::app::{AppState, ViewState};

pub enum InputState {
    Table,
    Column,
}

pub struct Cursor {
    pub x: usize,
    pub y: usize,
    // pub current: (usize, usize),
}

pub struct Db {
    pub db_name: Option<String>,
    pub table_name: String,
    /// Raw input separated by comma
    pub column: String,
    /// Attributes, containing constraints and other meta data
    pub parsed_column: Vec<Vec<String>>,
    /// Clean list of current column names
    pub col_names: Vec<String>,
    /// List of rows
    pub records: Vec<Vec<String>>,
    pub input_state: InputState,
}

impl Db {
    pub fn new() -> rusqlite::Result<Self> {
        Ok(Self {
            db_name: None,
            table_name: String::from(""),
            column: String::from(""),
            parsed_column: vec![],
            col_names: vec![],
            records: vec![],
            input_state: InputState::Table,
        })
    }

    pub fn open_db_if_exists(&self) -> Result<(), io::Error> {
        let name =
            Self::ensure_correct_path(self.db_name.clone().expect("Cannot open nameless DB file"));

        let result = File::open(name.trim());
        match result {
            Ok(_) => return Ok(()),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    self.create_db();
                    return Ok(());
                }
                return Err(err);
            }
        };
    }

    pub fn create_db(&self) -> Result<(), io::Error> {
        let name = Self::ensure_correct_path(self.db_name.clone().unwrap_or("default".to_owned()));

        match File::create(name.trim()) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        };
    }

    pub fn parse_table(&mut self) -> io::Result<()> {
        self.parsed_column = self
            .column
            .trim()
            .split(",")
            .map(|col| {
                col.split_whitespace()
                    .map(|constraint| constraint.to_owned())
                    .collect()
            })
            .collect();
        self.col_names = self
            .parsed_column
            .iter()
            .filter_map(|elm| elm.first().cloned())
            .collect();
        let conn =
            Connection::open(Self::ensure_correct_path(self.db_name.clone().unwrap())).unwrap();
        // println!("{}", self.col_names.join(","));
        conn.execute(
            &format!(
                "create table {} ({})",
                self.table_name,
                self.col_names.join(",")
            ),
            (),
        )
        .unwrap();
        Ok(())
    }

    // pub fn update_table(&self) {}

    pub fn add_record(&mut self) {
        // initialize vec with empty strings
        self.records
            .push(vec![String::from("hello"); self.col_names.len()]);
    }

    pub fn pop_record(&mut self) {
        self.records.pop();
    }

    pub fn select_table(&self) {}

    pub fn toggle_input_state(&mut self) {
        match self.input_state {
            InputState::Table => self.input_state = InputState::Column,
            InputState::Column => self.input_state = InputState::Table,
        }
    }

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

    use rusqlite::{types::Value, ToSql};

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

    #[test]
    fn type_to_sql_type() {
        assert_eq!(Value::Integer(10), Value::from(10));
    }
}
