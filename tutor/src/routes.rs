use crate::handlers::{get_course_details, get_courses_for_tutor, health_check, post_new_course};
use actix_web::web;

pub fn general(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub fn course(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_details)),
    );
}
