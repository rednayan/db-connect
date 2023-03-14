use actix_web::{get,post,web,Responder,HttpResponse};
use serde::{Serialize};
use super::models::{CreateTaskList};
use crate::AppState;
use std::process::exit;

#[derive(Serialize)]
struct TaskList {
    id: u32,
    title: String,
    content: String,
}


#[get("/task_list")]
async fn get_task_list(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.pool;
    let task_list = sqlx::query_as::<_,(u32,String,String)>("SELECT id,title,contents FROM task_list")
        .fetch_all(&*pool)
        .await;
    let data = match task_list {
        Ok(data) => data,
        Err(_) => exit(1)
    };
    let mut task_pool : Vec<TaskList> = Vec::new();
    for task in data {  
        task_pool.push(TaskList {
            id: task.0,
            title:task.1,
            content: task.2
        });
    }

    return HttpResponse::Ok().json(task_pool);
}   

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_task_list);
} 