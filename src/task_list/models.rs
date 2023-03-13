use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskList {
    pub title: String,
    pub contents: String,
    pub date: i64,
    pub time: i64,
}