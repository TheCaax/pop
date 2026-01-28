use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

pub struct FileRecord {
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub last_modified: i64,
    pub is_dir: bool,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                extension TEXT,
                size INTEGER NOT NULL,
                last_modified INTEGER NOT NULL,
                is_dir BOOLEAN NOT NULL
            )",
            [],
        )?;

        // Create indexes for fast searching
        conn.execute("CREATE INDEX IF NOT EXISTS idx_name ON files(name)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_extension ON files(extension)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_size ON files(size)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_last_modified ON files(last_modified)", [])?;

        // Optimization pragmas
        conn.execute("PRAGMA synchronous = OFF", [])?;
        let _ : String = conn.query_row("PRAGMA journal_mode = MEMORY", [], |row| row.get(0))?;

        Ok(Database { conn })
    }

    pub fn insert_batch(&mut self, records: &[FileRecord]) -> Result<()> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare_cached(
                "INSERT OR REPLACE INTO files (path, name, extension, size, last_modified, is_dir) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )?;
            for record in records {
                stmt.execute(params![
                    record.path,
                    record.name,
                    record.extension,
                    record.size,
                    record.last_modified,
                    record.is_dir
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        self.conn.execute("DELETE FROM files", [])?;
        Ok(())
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}
