use actix_web::{get,post,web,Responder,HttpResponse};
use super::models::{CreateTaskList};

#[get("/task_list")]
async fn get_task_list() -> String {
   
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_task_list);
} 