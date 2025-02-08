mod models;
mod services;

use services::t_services::TaskManager;
use services::t_storage::TaskStorage;
use models::task::Task;
use std::io::{self, Write};

fn main() {
    let task_storage = TaskStorage{};
    
    let file_path = "tasks.json";
    let mut tasks = match task_storage.read_tasks(file_path) {
        Ok(tasks) => tasks,
        Err(_) => {
            println!("Failed to read tasks from file. Starting new an empty list");
            Vec::new()
        }
    };

    let task_manager = TaskManager{tasks: vec![], next_id: 1};


    loop{
        println!("Choose an option:");
        println!("1. Add task");
        println!("2. Delete task");
        println!("3. View tasks");
        println!("4. Save and Exit");

        let choice = read_input();

        match choice {
            1 => {
                let task = create_task();
                task_manager.add_task(&mut tasks, task);
            }
            2 => {
                println!("Enter the task ID to delete:");
                let id = read_input();
                delete_task(&mut tasks, id);
            }
            3 => {
                list_tasks(&tasks);
            }
            4 => {
                if let Err(e) = write_tasks(file_path, &tasks) {
                    println!("Error saving tasks: {}", e);
                }
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn read_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().unwrap_or(0)
}

fn create_task() -> Task {
    println!("Enter task ID:");
    let id = read_input();
    println!("Enter task description:");
    let description = read_line();
    Task { id, description }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}