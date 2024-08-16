use std::{
    fmt::Debug,
    fs::File,
    io::{self, ErrorKind},
};

// pub trait Printable {
//     fn print(&self);
// }

// impl Printable for i32 {
//     fn print(&self) {
//         println!("am a number");
//     }
// }

pub trait SqlDataType: Debug {}

impl SqlDataType for i32 {}
impl SqlDataType for String {}
impl SqlDataType for char {}
impl SqlDataType for bool {}

pub struct Db {
    pub records: Vec<Box<dyn SqlDataType>>,
}

impl Db {
    pub fn add_record(&mut self) {
        for item in self.records.iter() {
            println!("{:?}", item);
        }
    }
}

pub fn create_db(name: &str) -> Result<(), io::Error> {
    let name = ensure_correct_path(name.to_owned());

    match File::create(name.trim()) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    };
}

pub fn open_db_if_exists(name: &str) -> Result<(), io::Error> {
    let name = ensure_correct_path(name.to_owned());

    let result = File::open(name.trim());
    match result {
        Ok(_) => return Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                return Err(err);
            }

            _ => return Err(err),
        },
    };
}

fn ensure_correct_path(mut name: String) -> String {
    if !name.ends_with(".db") {
        name.push_str(".db");
    }
    name
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, path::Path};

    #[test]
    fn create_db_file_from_path() {
        let path = "test.db";
        super::create_db(path).expect("Failed to create a new DB file with name: {}");

        assert!(Path::new(path).exists());

        remove_file(path).expect("Failed to remove DB file");
    }
}
