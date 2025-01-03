use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{Write};

const TODO_FILE: &str = "todo.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args]", args[0]);
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "add" => add_todo(&args[2..]),
        "list" => list_todos(),
        "remove" => remove_todo(&args[2..]),
        _ => eprintln!("Unknown command: {}", command),
    }
}


fn add_todo(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: todo-cli add <task>");
        return;
    }

    let task = args.join(" ");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(TODO_FILE)
        .expect("Failed to open file");

    writeln!(file, "{}", task).expect("Failed to write to file");
    println!("Added task: {}", task);
}


fn list_todos() {
    let contents = read_to_string(TODO_FILE).unwrap_or_else(|_| String::new());
    if contents.is_empty() {
        println!("No tasks found.");
    } else {
        for (index, task) in contents.lines().enumerate() {
            println!("{}: {}", index + 1, task);
        }
    }
}



fn remove_todo(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: todo-cli remove <task number>");
        return;
    }

    let task_number: usize = args[0].parse().expect("Please provide a valid number");
    let contents = read_to_string(TODO_FILE).unwrap_or_else(|_| String::new());

    let tasks: Vec<&str> = contents.lines().collect();
    if task_number == 0 || task_number > tasks.len() {
        eprintln!("Invalid task number.");
        return;
    }

    let updated_tasks: Vec<&str> = tasks.into_iter().enumerate()
        .filter(|&(index, _)| index != task_number - 1)
        .map(|(_, task)| task)
        .collect();

    std::fs::write(TODO_FILE, updated_tasks.join("\n")).expect("Failed to write to file");
    println!("Removed task {}", task_number);
}

