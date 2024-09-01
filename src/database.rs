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
    // x coord
    pub row: usize,
    // y coord
    pub col: usize,
    /// List of rows
    pub records: Vec<Vec<String>>,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            records: Vec::new(),
        }
    }

    pub fn next_col(&mut self) {
        if self.col < self.records[self.row].len() - 1 {
            self.col += 1;
        } else {
            self.col = 0; // cycle effect
        }
    }

    pub fn prev_col(&mut self) {
        if self.col == 0 {
            self.col = self.records[self.row].len() - 1;
        } else {
            self.col -= 1;
        }
    }

    pub fn next_row(&mut self) {
        if self.row < self.records.len() - 1 {
            self.row += 1;
        } else {
            self.row = 0;
        }
    }
    pub fn prev_row(&mut self) {
        if self.row == 0 {
            self.row = self.records.len() - 1;
        } else {
            self.row -= 1;
        }
    }

    pub fn get_selected_item(&self) -> &str {
        &self.records[self.row][self.col]
    }

    pub fn update_item(&mut self, item: char) {
        self.records[self.row][self.col].push(item);
        // println!("{:?}", self.records[self.row][self.col]);
    }

    pub fn pop_item(&mut self) {
        self.records[self.row][self.col].pop();
    }
}

pub struct Db {
    pub conn: Option<Connection>,
    pub db_name: Option<String>,
    pub table_name: String,
    /// Raw input separated by comma
    pub column: String,
    /// Temporary assigned value, usually used in Update state
    pub col_name: String,
    /// Attributes, containing constraints and other meta data
    pub parsed_column: Vec<Vec<String>>,
    /// Clean list of current column names
    pub col_names: Vec<String>,
    pub cursor: Cursor,
    pub input_state: InputState,
}

impl Db {
    pub fn new() -> rusqlite::Result<Self> {
        Ok(Self {
            conn: None,
            db_name: None,
            table_name: String::new(),
            column: String::new(),
            col_name: String::new(),
            parsed_column: Vec::new(),
            col_names: Vec::new(),
            cursor: Cursor::new(),
            input_state: InputState::Table,
        })
    }

    pub fn set_db(&mut self, name: String) {
        self.db_name = Some(name);
        self.conn = Some(
            Connection::open(Self::ensure_correct_path(self.db_name.clone().unwrap())).unwrap(),
        );
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
                    // todo: handle error
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
        // println!("{}", self.col_names.join(","));
        if let Some(conn) = self.conn.as_ref() {
            conn.execute(
                &format!(
                    "create table {} ({})",
                    self.table_name,
                    self.col_names.join(",")
                ),
                (),
            )
            .unwrap();
            // todo: handle error
        }
        Ok(())
    }

    //pub fn update_table(&self) {}
    pub fn add_column(&mut self, val: String) {
        if let Some(conn) = self.conn.as_ref() {
            conn.execute(
                &format!("alter table {} add column {}", self.table_name, val),
                (),
            )
            .unwrap();
            // todo: handle error
        }
        self.col_names
            .push(val.split_whitespace().collect::<Vec<&str>>()[0].to_owned());

        for row in self.cursor.records.iter_mut() {
            row.push(String::from(""));
        }
    }

    // fn fill_empty_col(&self) {}

    pub fn add_record(&mut self) {
        self.cursor
            .records
            .push(vec![String::from(""); self.col_names.len()]);
        // println!("Records after adding: {:?}", self.cursor.records);
        // println!("{:?}", self.cursor.records[0].len());
    }

    pub fn pop_record(&mut self) {
        self.cursor.records.pop();
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

    // #[test]
    // fn create_db_file_from_path() {
    //     let path = "test.db";
    //     super::Db::new()
    //         .unwrap()
    //         .create_db()
    //         .expect("Failed to create a new DB file with name: {}");

    //     assert!(Path::new(path).exists());

    //     remove_file(path).expect("Failed to remove DB file");
    // }

    #[test]
    fn type_to_sql_type() {
        assert_eq!(Value::Integer(10), Value::from(10));
    }
}
