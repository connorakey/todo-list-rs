use std::{env, path::Path};
use todo_list::Todo;

fn get_db_path() -> Option<&'static Path> {
    None
}

fn print_help() {
    println!("Usage:");
    println!("  todo                 # List all todos");
    println!("  todo help            # Show this help message");
    println!("  todo add <task1> ... # Add one or more tasks");
    println!("  todo remove <id>     # Remove a task by ID");
    println!("  todo complete <id>   # Mark a task as complete");
    println!("  todo purge           # Delete the database");
}

fn list_todos(db_path: Option<&Path>) {
    match Todo::list(db_path) {
        Ok(todos) => {
            if todos.is_empty() {
                println!("No todos found.");
            } else {
                for (id, todo) in todos.iter() {
                    println!(
                        "{}. {} [{}]",
                        id,
                        todo.entry,
                        if todo.done { "x" } else { " " }
                    );
                }
            }
        }
        Err(e) => eprintln!("Failed to list todos: {}", e),
    }
}

fn main() {
    let db_path = get_db_path();

    if let Err(e) = Todo::init(db_path) {
        eprintln!("Failed to initialize database: {}", e);
        return;
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        list_todos(db_path);
        return;
    }

    let command = &args[1];

    match &command[..] {
        "help" => print_help(),

        "list" => list_todos(db_path),

        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add <task1> [task2] ...");
            } else {
                for arg in &args[2..] {
                    let _ = Todo::new(arg.to_string(), false).save(db_path);
                }
            }
        }

        "remove" => {
            if args.len() != 3 {
                println!("Usage: todo remove <id>");
            } else if let Ok(id) = args[2].parse::<i32>() {
                let _ = Todo::remove(id, db_path);
            } else {
                println!("Invalid id: {}", args[2]);
            }
        }

        "complete" => {
            if args.len() < 3 {
                println!("Usage: todo complete <id1> [id2] ...");
            } else {
                for arg in &args[2..] {
                    if let Ok(id) = arg.parse::<i32>() {
                        let _ = Todo::complete(id, db_path);
                    } else {
                        println!("Invalid id: {}", arg);
                    }
                }
            }
        }

        "purge" => {
            let _ = Todo::purge(db_path);
        }

        _ => {
            println!("Unknown command: {}", command);
            print_help();
        }
    }
}
