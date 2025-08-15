use todo_list::Todo;
use std::fs;
use serial_test::serial;

fn reset_db() {
    let _ = fs::remove_file("todo_list_database.db");
    Todo::init().unwrap();
}

#[test]
#[serial]
fn test_init_and_empty_list() {
    reset_db();
    let todos = Todo::list().unwrap();
    assert!(todos.is_empty(), "Database should start empty");
}

#[test]
#[serial]
fn test_add_and_list() {
    reset_db();
    Todo::new("Task 1".to_string(), false).save().unwrap();
    Todo::new("Task 2".to_string(), false).save().unwrap();

    let todos = Todo::list().unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].1.entry, "Task 1");
    assert_eq!(todos[1].1.entry, "Task 2");
    assert!(!todos[0].1.done);
    assert!(!todos[1].1.done);
}

#[test]
#[serial]
fn test_complete_task() {
    reset_db();
    Todo::new("Task 1".to_string(), false).save().unwrap();
    Todo::complete(1).unwrap();

    let todos = Todo::list().unwrap();
    assert!(todos[0].1.done);
}

#[test]
#[serial]
fn test_remove_task() {
    reset_db();
    Todo::new("Task 1".to_string(), false).save().unwrap();
    Todo::new("Task 2".to_string(), false).save().unwrap();

    Todo::remove(1).unwrap();
    let todos = Todo::list().unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].0, 1);
    assert_eq!(todos[0].1.entry, "Task 2");
}

#[test]
#[serial]
fn test_multiple_operations() {
    reset_db();

    Todo::new("Task A".to_string(), false).save().unwrap();
    Todo::new("Task B".to_string(), false).save().unwrap();
    Todo::new("Task C".to_string(), false).save().unwrap();

    Todo::complete(2).unwrap();
    Todo::remove(1).unwrap();

    let todos = Todo::list().unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].0, 1);
    assert_eq!(todos[0].1.entry, "Task B");
    assert!(todos[0].1.done);
    assert_eq!(todos[1].0, 2);
    assert_eq!(todos[1].1.entry, "Task C");
    assert!(!todos[1].1.done);
}
