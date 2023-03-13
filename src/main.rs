use sqlx::{MySqlPool};
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://root:deadmanalive@localhost:3306/demo").await?;

    let names = sqlx::query_as::<_, (String,)>("SELECT name FROM users")
        .fetch_all(&pool)
        .await?;

    for row in names {
        let name = row.0;
        println!("Name: {}", name);
    }

    Ok(())
}