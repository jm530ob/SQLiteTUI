use std::{
    fs::File,
    io::{self, ErrorKind},
};

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
