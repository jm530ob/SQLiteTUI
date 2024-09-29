use std::io;

use app::App;
use clap::{Args, Parser};
use rusqlite::{params, Connection, Result};

mod app;
mod components;
// mod database;
mod tui;
// mod ui;
mod models;
mod utils;

fn main() -> io::Result<()> {
    let args = models::args::Args::parse();
    let mut terminal = tui::init()?;
    let mut app = App::new();
    app.setup(args);
    app.run(&mut terminal)?;
    tui::clear()?;
    // let mut test = database::Db {
    //     records: vec![Box::new(20)],
    // };
    // test.add_record();

    // let conn = Connection::open_in_memory()?;

    // conn.execute(
    //     "CREATE TABLE person (
    //         id   INTEGER PRIMARY KEY,
    //         name TEXT NOT NULL,
    //         data BLOB
    //     )",
    //     (), // empty list of parameters.
    // )?;
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     (&me.name, &me.data),
    // )?;

    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     (&me.name, &me.data),
    // )?;

    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;

    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }
    Ok(())
}
