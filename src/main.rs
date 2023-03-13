

// #[derive(Debug)]
// struct Name {
//     name: Vec<String>,
// }

// #[actix_web::main]
// async fn main() -> Result<(), sqlx::Error> {
//     let pool = MySqlPool::connect("mysql://root:deadmanalive@localhost:3306/demo").await?;

//     let names = sqlx::query_as::<_, (String,)>("SELECT name FROM users")
//         .fetch_all(&pool)
//         .await?;
//     let mut vec_string: Vec<String> = Vec::new();

//     for row in names {
//         vec_string.push(row.0.to_string());
//     }
//     let name_struct = Name {
//         name : vec_string,
//     };

//     println!("{:?}",name_struct);

//     Ok(())
// }
use actix_web::{get,web,HttpServer,App};
use sqlx::{MySqlPool};

mod task_list;
use task_list::services;
use std::process::exit;


struct AppState {
    pool: MySqlPool
}

#[get("/")]
async fn index() -> String {
    format!("This is it")
} 

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let pool = MySqlPool::connect("mysql://root:deadmanalive@localhost:3306/demo").await;
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


