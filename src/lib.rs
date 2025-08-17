use dirs;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Todo {
    pub entry: String,
    pub done: bool,
}

impl Todo {
    pub fn init(db_path: Option<&Path>) -> Result<()> {
        create_database(db_path)?;
        Ok(())
    }

    pub fn new(entry: String, done: bool) -> Todo {
        Todo { entry, done }
    }

    pub fn save(&self, db_path: Option<&Path>) -> Result<()> {
        let conn = Connection::open(get_db_path_opt(db_path)?)?;
        conn.execute(
            "INSERT INTO todo (task, completed) VALUES (?1, ?2)",
            params![self.entry, self.done],
        )?;
        Ok(())
    }

    pub fn list(db_path: Option<&Path>) -> Result<Vec<(i32, Todo)>> {
        let conn = Connection::open(get_db_path_opt(db_path)?)?;
        let mut stmt = conn.prepare("SELECT id, task, completed FROM todo ORDER BY id")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                entry: row.get::<_, String>(1)?,
                done: row.get::<_, bool>(2)?,
            })
        })?;

        let mut todos = Vec::new();
        for (index, item) in todo_iter.enumerate() {
            let todo = item?;
            todos.push(((index + 1) as i32, todo));
        }
        Ok(todos)
    }

    pub fn remove(id: i32, db_path: Option<&Path>) -> Result<()> {
        let conn = Connection::open(get_db_path_opt(db_path)?)?;
        let mut stmt = conn.prepare("SELECT id FROM todo ORDER BY id")?;
        let ids_iter = stmt.query_map([], |row| row.get::<_, i32>(0))?;
        let db_ids: Vec<i32> = ids_iter.collect::<Result<Vec<i32>>>()?;

        if let Some(&db_id) = db_ids.get((id - 1) as usize) {
            conn.execute("DELETE FROM todo WHERE id = ?1", params![db_id])?;
        }
        Ok(())
    }

    pub fn complete(id: i32, db_path: Option<&Path>) -> Result<()> {
        let conn = Connection::open(get_db_path_opt(db_path)?)?;
        let mut stmt = conn.prepare("SELECT id FROM todo ORDER BY id")?;
        let ids_iter = stmt.query_map([], |row| row.get::<_, i32>(0))?;
        let db_ids: Vec<i32> = ids_iter.collect::<Result<Vec<i32>>>()?;

        if let Some(&db_id) = db_ids.get((id - 1) as usize) {
            conn.execute(
                "UPDATE todo SET completed = 1 WHERE id = ?1",
                params![db_id],
            )?;
        }
        Ok(())
    }

    pub fn purge(db_path: Option<&Path>) -> io::Result<()> {
        let database = get_db_path_opt_io(db_path)?;
        if database.exists() {
            fs::remove_file(database)?;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Database file does not exist",
            ))
        }
    }
}

/// Returns the default database path
pub fn get_db_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir().ok_or_else(|| rusqlite::Error::InvalidPath(PathBuf::new()))?;
    path.push("todo-list-rs");
    fs::create_dir_all(&path).map_err(|_| rusqlite::Error::InvalidPath(path.clone()))?;
    path.push("todo_list.db");
    Ok(path)
}

fn get_db_path_opt(db_path: Option<&Path>) -> Result<PathBuf> {
    match db_path {
        Some(p) => Ok(p.to_path_buf()),
        None => get_db_path(),
    }
}

fn get_db_path_opt_io(db_path: Option<&Path>) -> io::Result<PathBuf> {
    match db_path {
        Some(p) => Ok(p.to_path_buf()),
        None => {
            let mut path = dirs::data_dir().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Could not find data dir")
            })?;
            path.push("todo-list-rs");
            fs::create_dir_all(&path)?;
            path.push("todo_list.db");
            Ok(path)
        }
    }
}

fn create_database(db_path: Option<&Path>) -> Result<()> {
    let conn = Connection::open(get_db_path_opt(db_path)?)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            completed BOOL NOT NULL
        )",
        [],
    )?;
    Ok(())
}
