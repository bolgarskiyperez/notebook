
use crate::models::task::Task;

pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub next_id: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: &str) -> Task {
        let task = Task::new(self.next_id, title, false);
        self.tasks.push(task);
        self.next_id += 1;
        self.tasks.last().unwrap().clone()
        /*unwrap as error flag(option Some or None(err)), 
        the task as just been added therefore last unwrap will find it
        clone for return the copy of task(task.rs must contains #[derive(Clone)])*/
    }

    pub fn delete_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id !=id);
        /*retain for save vec element of collection, that eligible
        |task| task.id !=id it is iterate all elements  of vec and return bool result
        (compare id and task.id)*/ 
    }

    pub fn get_task(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|&task| task.id == id)
    }
    // Option need for return option result(Some(&task) or None)
     
}