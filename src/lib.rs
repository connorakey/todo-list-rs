use rusqlite::{Connection, Result, params};

pub struct Todo {
    pub entry: String,
    pub done: bool,
}

impl Todo {
    pub fn init() -> Result<()> {
        create_database()?;
        Ok(())
    }

    pub fn new(entry: String, done: bool) -> Todo {
        Todo { entry, done }
    }

    pub fn save(&self) -> Result<()> {
        let conn = Connection::open("todo_list_database.db")?;
        conn.execute(
            "INSERT INTO todo (task, completed) VALUES (?1, ?2)",
            params![self.entry, self.done],
        )?;
        Ok(())
    }

    pub fn list() -> Result<Vec<(i32, Todo)>> {
        let conn = Connection::open("todo_list_database.db")?;
        let mut stmt = conn.prepare("SELECT task, completed FROM todo ORDER BY id")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                entry: row.get::<_, String>(0)?,
                done: row.get::<_, bool>(1)?,
            })
        })?;

        let mut todos = Vec::new();
        for (index, item) in todo_iter.enumerate() {
            let todo = item?;
            todos.push(((index + 1) as i32, todo));
        }
        Ok(todos)
    }

    pub fn remove(id: i32) -> Result<()> {
        let conn = Connection::open("todo_list_database.db")?;
        let mut stmt = conn.prepare("SELECT id FROM todo ORDER BY id")?;
        let ids_iter = stmt.query_map([], |row| row.get::<_, i32>(0))?;
        let db_ids: Vec<i32> = ids_iter.collect::<Result<Vec<i32>>>()?;

        if let Some(&db_id) = db_ids.get((id - 1) as usize) {
            conn.execute("DELETE FROM todo WHERE id = ?1", params![db_id])?;
        }
        Ok(())
    }

    pub fn complete(id: i32) -> Result<()> {
        let conn = Connection::open("todo_list_database.db")?;
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
}

fn create_database() -> Result<()> {
    let conn = Connection::open("todo_list_database.db")?;
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
