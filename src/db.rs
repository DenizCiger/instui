use rusqlite::{ params, Connection, Result };
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS session (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                username TEXT NOT NULL,
                token TEXT NOT NULL
            )",
            []
        )?;
        Ok(())
    }

    pub fn save_session(&self, username: &str, token: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO session (id, username, token) VALUES (1, ?1, ?2)",
            params![username, token]
        )?;
        Ok(())
    }

    pub fn get_session(&self) -> Result<Option<(String, String)>> {
        let mut stmt = self.conn.prepare("SELECT username, token FROM session WHERE id = 1")?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?)))
        } else {
            Ok(None)
        }
    }

    pub fn clear_session(&self) -> Result<()> {
        self.conn.execute("DELETE FROM session WHERE id = 1", [])?;
        Ok(())
    }
}
