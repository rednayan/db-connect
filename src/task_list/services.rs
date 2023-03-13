use actix_web::{get,post,web,Responder,HttpResponse};
use serde::{Deserialize,Serialize};
use super::models::{CreateTaskList};
use crate::AppState;
use std::process::exit;

#[derive(Serialize,Deserialize)]
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
        Ok(data) => println!("data is in here : {:?}",data),
        Error => println!("error pool",)
    };

    let task_pool = TaskList {
        id:1,
        title:"here".to_string(),
        content:"this is awesome".to_string(),
    };
    return HttpResponse::Ok().json(task_pool);
}   

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_task_list);
} 