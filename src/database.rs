use std::{
    fs::File,
    io::{self, Error, ErrorKind},
};

pub fn create_db(name: &str) -> Result<(), io::Error> {
    let mut name = name.to_owned();
    name.push_str(".db");

    match File::create(name.trim()) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    };
}

pub fn open_db_if_exists(name: &str) -> Result<(), io::Error> {
    todo!("handle .db file extension as well as without it");
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
