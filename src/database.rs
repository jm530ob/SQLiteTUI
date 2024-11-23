use rusqlite::{params, Connection, Result};

//#[derive(Clone, Copy)]
pub struct Database {
    pub conn: Connection,
    pub table: Option<String>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn, table: None })
    }

    pub fn list_tables(&self, conn: &Connection) -> Result<Vec<String>> {
        let mut stmt = conn.prepare("select name from sqlite_master where type='table'")?;

        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut tables = vec![];
        for row in rows {
            tables.push(row?);
        }

        Ok(tables)
    }

    pub fn get_query(&self, conn: &Connection, table: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(&format!("select * from {}", table))?;
        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut data = vec![];
        for row in rows {
            data.push(row?);
        }

        Ok(data)
    }

    pub fn column_names(&self, conn: &Connection, table: &str) -> Result<(i16, Vec<String>)> {
        let mut stmt = conn.prepare(&format!("select * from {}", table))?;
        stmt.column_names().iter()
        Ok()
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
