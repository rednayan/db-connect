use actix_web::{get,web,HttpServer,App};
use sqlx::{MySqlPool};
use dotenv::dotenv;

mod task_list;
use task_list::services;
use std::process::exit;


struct AppState {
    pool: MySqlPool
}

#[get("/")]
async fn index() -> String {
    format!("go to /task_list")
} 

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    
    let database_url: String = std::env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url).await;
    let app_data = web::Data::new(AppState {pool : match pool {
        Ok(val) => val,
        Err(_) => exit(1)
    }});
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
        .bind(("127.0.0.1",3000))?
        .run()
        .await
}


