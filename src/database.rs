use std::{
    fs::File,
    io::{self, Error},
};

pub fn create_db(name: &str) -> io::Result<()> {
    if !name.trim().ends_with(".db") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File name must end with .db",
        ));
    }
    let db = File::create(name.trim())?;
    Ok(())
}

pub fn locate_db(path: &str) -> io::Result<()> {
    if let Ok(_) = File::open(path.trim()) {
        return Ok(());
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Database: {} was not found!", path),
        ));
    }
}
