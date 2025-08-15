use todo_list::Todo;
use std::env;

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
        // No arguments, just list todos
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
                    println!("Added: {}", arg);
                }
            }
        }

        "remove" => {
            if args.len() != 3 {
                println!("Usage: todo remove <id>");
            } else if let Ok(id) = args[2].parse::<i32>() {
                Todo::remove(id).unwrap();
                println!("Removed todo #{}", id);
            } else {
                println!("Invalid id: {}", args[2]);
            }
        }

        "complete" => {
            if args.len() != 3 {
                println!("Usage: todo complete <id>");
            } else if let Ok(id) = args[2].parse::<i32>() {
                Todo::complete(id).unwrap();
                println!("Marked todo #{} as complete", id);
            } else {
                println!("Invalid id: {}", args[2]);
            }
        }

        _ => {
            println!("Unknown command: {}", command);
            print_help();
        }
    }
}
