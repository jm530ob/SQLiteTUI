use std::{
    fmt::Debug,
    fs::File,
    io::{self, ErrorKind},
};

use rusqlite::{types, Connection};

// pub trait SqlDataType: Debug {}

// impl SqlDataType for i32 {}
// impl SqlDataType for f32 {}
// impl SqlDataType for String {}
// impl SqlDataType for char {}
// impl SqlDataType for bool {}

// pub type SqlTypeObj = Box<dyn SqlDataType>;

pub struct Cursor {
    pub x: usize,
    pub y: usize,
    // pub current: (usize, usize),
}

pub struct Db {
    pub name: Option<String>,
    pub records: Vec<types::Value>,
}

impl Db {
    pub fn new(path: &str) -> rusqlite::Result<Self> {
        Ok(Self {
            name: Some(path.to_owned()),
            records: vec![],
        })
    }

    /// static function that opens db file if it exists
    pub fn open_db_if_exists(name: &str) -> Result<Self, io::Error> {
        let name = Self::ensure_correct_path(name.to_owned());

        let result = File::open(name.trim());
        match result {
            Ok(_) => return Ok(Self::new(&name).expect("Failed to open a DB")),

            Err(err) => return Err(err),
        };
    }

    pub fn create_db(&self) -> Result<(), io::Error> {
        let name = Self::ensure_correct_path(self.name.clone().unwrap_or("default".to_owned()));

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

    pub fn store_table(&self) -> rusqlite::Result<()> {
        //hmmm neunpackujem?
        let conn = Connection::open(Self::ensure_correct_path(
            self.name.clone().expect("DB file not specified"),
        ))?;
        conn.execute(
            "create table if not exists person (id integer primary key, name text)",
            (),
        )?;

        conn.execute("insert into person (id, name) values (?1, ?2)", (1, "Test"))?;

        let mut stmt = conn.prepare("select name from sqlite_master where type='table'")?;
        let iter = stmt.query_map([], |row| {
            let jupi: String = row.get(0)?;
            Ok(jupi)
        })?;

        for i in iter {
            println!("{:?}", i.unwrap());
        }

        Ok(())
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

    #[test]
    fn create_db_file_from_path() {
        let path = "test.db";
        super::Db::new(path)
            .unwrap()
            .create_db()
            .expect("Failed to create a new DB file with name: {}");

        assert!(Path::new(path).exists());

        remove_file(path).expect("Failed to remove DB file");
    }
}
