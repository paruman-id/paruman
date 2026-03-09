use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Running migrations against {}", database_url);
    sqlx::migrate!("../../backend/migrations")
        .run(&sqlx::PgPool::connect(&database_url).await?)
        .await?;
    println!("Migrations completed");
    Ok(())
}
