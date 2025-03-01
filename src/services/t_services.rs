use super::t_storage::{self, TaskStorage};
use crate::models::task::Task;

pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub next_id: usize,
    pub task_storage: TaskStorage,  // Добавляем поле
}

impl TaskManager {
    pub fn new(task_storage: TaskStorage) -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
            task_storage,  // Инициализируем task_storage
        }
    }

    pub fn add_task(&mut self, title: &str, description: String) {
        let task = Task::new(self.next_id, title, false, description);
        self.tasks.push(task);
        self.next_id += 1;
        self.task_storage.write_tasks("tasks.json", &self.tasks).expect("Failed to save tasks");
    }

    pub fn delete_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
        self.task_storage.write_tasks("tasks.json", &self.tasks).expect("Failed to save tasks");
    }

    pub fn get_task(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|&task| task.id == id)
    }
}
