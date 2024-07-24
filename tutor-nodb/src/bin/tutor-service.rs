use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;
use tutor_nodb::routes;
use tutor_nodb::state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(state::App {
        health_check_response: "I'm good. You've already asked me".to_owned(),
        visit_count: AtomicU32::new(0),
        courses: Mutex::new(Vec::new()),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(routes::general)
            .configure(routes::course)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
