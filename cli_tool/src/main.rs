use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let conn = Connection::open("persons.db")?;

    match args.get(1).map(String::as_str) {
        Some("create") => create_table(&conn),
        Some("insert") => insert_person(&conn, &args[2], args[3].parse().unwrap()),
        Some("read") => read_persons(&conn),
        Some("update") => update_person(&conn, &args[2], args[3].parse().unwrap()),
        Some("delete") => delete_person(&conn, &args[2]),
        _ => {
            println!("Invalid command");
            Ok(())
        }
    }
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS persons (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        NO_PARAMS,
    )?;
    Ok(())
}

fn insert_person(conn: &Connection, name: &str, age: i32) -> Result<()> {
    conn.execute("INSERT INTO persons (name, age) VALUES (?1, ?2)", params![name, age])?;
    Ok(())
}

fn read_persons(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, age FROM persons")?;
    let person_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
        ))
    })?;

    for person in person_iter {
        let (id, name, age) = person?;
        println!("ID: {}, Name: {}, Age: {}", id, name, age);
    }
    Ok(())
}

fn update_person(conn: &Connection, name: &str, age: i32) -> Result<()> {
    conn.execute("UPDATE persons SET age = ?1 WHERE name = ?2", params![age, name])?;
    Ok(())
}

fn delete_person(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("DELETE FROM persons WHERE name = ?1", params![name])?;
    Ok(())
}