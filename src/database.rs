use rusqlite::{params, Connection, Result};
pub struct Database {
    conn: Connection,

}
impl Database {

    pub fn new(&mut self, path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self {conn})
    }
    pub fn select_table(&self, table: &str) -> Result<()> {
        let stmt = self.conn.prepare("SELECT * FROM {table}")?;
        Ok(())
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
