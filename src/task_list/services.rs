use actix_web::{get,post,web,Responder,HttpResponse};
use super::models::{TaskList,CreateTaskList};
use crate::AppState;
use std::process::exit;


#[get("/task_list")]
async fn get_task_list(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.pool;
    let task_list = sqlx::query_as::<_,(u32,String,String)>("SELECT * FROM task_list")
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
            contents: task.2
        });
    }

    return HttpResponse::Ok().json(task_pool);
} 

#[post("/task_list")]
async fn post_task_list(data: web::Data<AppState>,param_obj: web::Json<CreateTaskList>) -> impl Responder {
    let pool = &data.pool;
    let query: String = format!("INSERT INTO task_list (title,contents) VALUES ('{}','{}')",param_obj.title.to_string(),param_obj.contents.to_string());
    sqlx::query(&query).execute(&*pool).await.expect("there is an error");
    return HttpResponse::Ok().json("Ok");
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_task_list)
       .service(post_task_list);
} 