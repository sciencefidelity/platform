use sqlx::postgres::PgPool;
use std::sync::atomic::AtomicU32;

pub struct App {
    pub health_check_response: String,
    pub visit_count: AtomicU32,
    pub db: PgPool,
}
