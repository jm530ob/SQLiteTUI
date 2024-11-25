use rusqlite::{types::ValueRef, Connection, Result};

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

    pub fn get_query(&self, conn: &Connection, table: &str) -> Result<Vec<Vec<String>>> {
        let mut stmt = conn.prepare(&format!("select * from {}", table))?;
        let column_count = stmt.column_count();
        let rows = stmt.query_map([], |row| {
            Ok((0..column_count)
                .map(|i| match row.get_ref(i).unwrap() {
                    ValueRef::Real(real) => real.to_string(),
                    ValueRef::Null => "NULL".to_string(),
                    ValueRef::Integer(int) => int.to_string(),
                    ValueRef::Text(text) => String::from_utf8(text.to_vec()).unwrap(),
                    ValueRef::Blob(blob) => String::from_utf8(blob.to_vec()).unwrap(),
                })
                .collect::<Vec<String>>())
        })?;
        let mut data = vec![];
        for row in rows {
            data.push(row?);
        }

        Ok(data)
    }

    pub fn column_names(&self, conn: &Connection, table: &str) -> Result<(usize, Vec<String>)> {
        let stmt = conn.prepare(&format!("select * from {}", table))?;
        let columns: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(|col: &str| col.to_owned())
            .collect();
        Ok((stmt.column_count(), columns))
    }

    pub fn max_len_str(
        &self,
        col_index: usize,
        col: &Vec<String>,
        rows: &Vec<Vec<String>>,
    ) -> usize {
        let mut max = 0;
        rows.iter().for_each(|row| {
            row.iter().enumerate().for_each(|(i, item)| {
                if i != col_index {
                    return;
                }

                if item.len() > max {
                    max = item.len();
                }
            });

            let col_len = col.get(col_index).unwrap().len();
            if max < col_len {
                max = col_len;
            }
        });

        return max + 2 as usize;
    }
}

#[cfg(test)]
mod tests {

    use rusqlite::types::Value;

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
