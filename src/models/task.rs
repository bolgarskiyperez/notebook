use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub task: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, title: &str, completed: bool) -> Self {
        Task {
            id,
            task: title.to_string(),
            completed,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}
