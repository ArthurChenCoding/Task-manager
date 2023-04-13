use std::io::{self, Write};

const TASKS_FILE: &str = "tasks.json";

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

      
fn main() {
    let mut task_manager = match TaskManager::load_from_file(TASKS_FILE) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            TaskManager::new()
        }
    };

    loop {
        print!("Enter command (add/remove/list/save/exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_parts: Vec<&str> = input.trim().split_whitespace().collect();

        if input_parts.is_empty() {
            continue;
        }

        match input_parts[0] {
            "add" => {
                if input_parts.len() < 2 {
                    println!("Usage: add <task description>");
                } else {
                    let description = input_parts[1..].join(" ");
                    task_manager.add_task(&description);
                    println!("Task added.");
                }
            }
            "remove" => {
                if input_parts.len() < 2 {
                    println!("Usage: remove <task id>");
                } else {
                    let id = input_parts[1].parse::<u32>().unwrap_or(0);
                    if task_manager.remove_task(id) {
                        println!("Task removed.");
                    } else {
                        println!("Task not found.");
                    }
                }
            }
            "list" => {
                task_manager.list_tasks();
            }
            "save" => {
                match task_manager.save_to_file(TASKS_FILE) {
                    Ok(_) => println!("Tasks saved."),
                    Err(e) => eprintln!("Error saving tasks: {}", e),
                }
            }
            "exit" => break,
            _ => println!("Unknown command. Use 'add', 'remove', 'list', 'save', or 'exit'."),
        }
    }
}
