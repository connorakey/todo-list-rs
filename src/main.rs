use std::env;
use todo_list::Todo;

fn print_help() {
    println!("Usage:");
    println!("  todo                 # List all todos");
    println!("  todo help            # Show this help message");
    println!("  todo add <task1> ... # Add one or more tasks");
    println!("  todo remove <id>     # Remove a task by ID");
    println!("  todo complete <id>   # Mark a task as complete");
}

fn list_todos() {
    let todos = Todo::list().unwrap();
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

fn main() {
    Todo::init().unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        list_todos();
        return;
    }

    let command = &args[1];

    match &command[..] {
        "help" => print_help(),

        "list" => list_todos(),

        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add <task1> [task2] ...");
            } else {
                for arg in &args[2..] {
                    Todo::new(arg.to_string(), false).save().unwrap();
                }
            }
        }

        "remove" => {
            if args.len() != 3 {
                println!("Usage: todo remove <id>");
            } else if let Ok(id) = args[2].parse::<i32>() {
                Todo::remove(id).unwrap();
            } else {
                println!("Invalid id: {}", args[2]);
            }
        }

        "complete" => {
            if args.len() < 3 {
                println!("Usage: todo complete <id1> [id2] ...");
            } else {
                for arg in &args[2..] {
                    match arg.parse::<i32>() {
                        Ok(id) => {
                            if let Err(e) = Todo::complete(id) {
                                println!("Failed to complete todo #{}: {}", id, e);
                            } else {
                                println!("Marked todo #{} as complete", id);
                            }
                        }
                        Err(_) => println!("Invalid id: {}", arg),
                    }
                }
            }
        }

        "purge" => if let Err(e) = Todo::purge() {
            println!("Failed to delete database: {}", e);
        }

        _ => {
            println!("Unknown command: {}", command);
            print_help();
        }
    }
}
