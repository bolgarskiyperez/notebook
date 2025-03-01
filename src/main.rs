mod models;
mod services;

use services::t_services::TaskManager;
use services::t_storage::TaskStorage;
use std::io::{self, Write};

fn main() {
    let task_storage = TaskStorage {};
    let mut task_manager = TaskManager::new(task_storage);

    let file_path = "tasks.json";
    let tasks = match task_manager.task_storage.read_tasks(file_path) {
        Ok(tasks) => tasks,
        Err(_) => {
            println!("Failed to read tasks from file. Starting with an empty list.");
            Vec::new()
        }
    };

    task_manager.tasks = tasks; // Присваиваем задачи из файла

    loop {
        println!("\nChoose an option:");
        println!("1. Add task");
        println!("2. Delete task");
        println!("3. View tasks");
        println!("4. Save and Exit");

        let choice = read_input();

        match choice {
            1 => {
                println!("Enter the title:");
                let title = read_line();

                println!("Enter the description:");
                let description = read_line();

                task_manager.add_task(&title, description);
            }
            2 => {
                println!("Enter the task ID to delete:");
                let id: usize = read_input();
                task_manager.delete_task(id);
            }
            3 => {
                println!("\nCurrent tasks:");
                for task in &task_manager.tasks {
                    println!(
                        "ID: {}, Title: {}, Completed: {}, Description: {}", 
                        task.id, task.title, task.completed, task.description
                    );
                }
            }
            4 => {
                task_manager.task_storage.write_tasks(file_path, &task_manager.tasks).expect("Error saving tasks");
                println!("Tasks saved. Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn read_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().unwrap_or(0)
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
