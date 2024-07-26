use crate::models::Course;
use crate::state;
use actix_web::{web, HttpResponse};
use std::sync::atomic::Ordering;

pub async fn health_check_handler(app_state: web::Data<state::App>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let visit_count = app_state.visit_count.fetch_add(1, Ordering::SeqCst);
    let response = format!("{health_check_response} {visit_count} times",);
    HttpResponse::Ok().json(&response)
}

/// # Panics
///
/// Will panic if `courses` fails to lock.
pub async fn get_courses_for_tutor(
    _app_state: web::Data<state::App>,
    _params: web::Path<i32>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

/// # Panics
///
/// Will panic if `courses` fails to lock.
pub async fn get_course_details(
    _app_state: web::Data<state::App>,
    _params: web::Path<(i32, usize)>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

/// # Panics
///
/// Will panic if `courses` fails to lock.
pub async fn post_new_course(
    _app_state: web::Data<state::App>,
    _new_course: web::Json<Course>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::atomic::AtomicU32;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<state::App> = web::Data::new(state::App {
            health_check_response: String::new(),
            visit_count: AtomicU32::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_details_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<state::App> = web::Data::new(state::App {
            health_check_response: String::new(),
            visit_count: AtomicU32::new(0),
            db: pool,
        });
        let params: web::Path<(i32, usize)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<state::App> = web::Data::new(state::App {
            health_check_response: String::new(),
            visit_count: AtomicU32::new(0),
            db: pool,
        });
        let new_course_msg = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "This is the next course".to_owned(),
            posted_time: Some(
                NaiveDate::from_ymd_opt(2024, 6, 17)
                    .expect("couldn't parse date")
                    .and_hms_opt(14, 01, 11)
                    .expect("couldn't parse time"),
            ),
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(app_state, course_param).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}