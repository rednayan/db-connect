use serde::{Serialize,Deserialize};

#[derive(Serialize)]
pub struct TaskList {
    pub id:u32,
    pub title:String,
    pub contents:String
}
#[derive(Deserialize)]
pub struct CreateTaskList {
    pub title:String,
    pub contents:String
}