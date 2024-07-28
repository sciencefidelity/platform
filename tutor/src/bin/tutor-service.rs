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
        health_check_response: "I'm good. You've already asked me".to_owned(),
        visit_count: AtomicU32::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(routes::general)
            .configure(routes::course)
    };

    let app_host = env::var("APP_HOST").expect("APP_HOST is not set in .env file");
    let app_port: u16 = env::var("APP_PORT")
        .expect("APP_PORT is not set in .env file")
        .parse()
        .expect("APP_PORT is not valid u16");
    HttpServer::new(app).bind((app_host, app_port))?.run().await
}
