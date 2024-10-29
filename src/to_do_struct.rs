use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ToDoStatus {
    Pending,
    Started,
    Completed,
    Cancelled,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoItem {
    pub title: String,
    pub description: String,
    pub status: ToDoStatus,
}
