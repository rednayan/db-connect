use actix_web::{get,post,web,Responder,HttpResponse};
use super::models::{CreateTaskList};
use crate::AppState;
use sqlx;

#[get("/task_list")]
async fn get_task_list(data: web::Data<AppState>) -> String {
    let pool = &data.pool;
    let task_list = sqlx::query_as::<_,(String,)>("SELECT title FROM task_list")
        .fetch_all(&*pool)
        .await;

    for task in task_list {
        println!("{:?}",task);
    }
    format!("working")
}   

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_task_list);
} 