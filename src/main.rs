use rusqlite::{Connection, Result};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    Ok(())
}
