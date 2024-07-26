use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::{env, io, sync::atomic::AtomicU32};
use tutor::{routes, state};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect to db");
    let shared_data = web::Data::new(state::App {
        health_check_response: "I'm good. You've already asked me ".to_owned(),
        visit_count: AtomicU32::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(routes::general)
            .configure(routes::course)
    };

    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}
