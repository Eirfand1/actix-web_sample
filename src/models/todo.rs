use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub task: String,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub task: String,
}
