use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub description: String
}

impl Task {
    pub fn new(id: usize, title: &str, completed: bool, description: String) -> Self {
        Task {
            id,
            title: title.to_string(),
            completed,
            description
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}
