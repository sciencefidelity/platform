use crate::models::Course;
use crate::state;
use actix_web::{web, HttpResponse};
use chrono::Utc;
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
pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<state::App>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .expect("failed to lock")
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state
        .courses
        .lock()
        .expect("failed to lock")
        .push(new_course);
    HttpResponse::Ok().json("Added course")
}

/// # Panics
///
/// Will panic if `courses` fails to lock.
pub async fn get_courses_for_tutor(
    app_state: web::Data<state::App>,
    params: web::Path<i32>,
) -> HttpResponse {
    let tutor_id = params.into_inner();
    let filtered_courses = app_state
        .courses
        .lock()
        .expect("failed to lock")
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();
    if filtered_courses.is_empty() {
        HttpResponse::Ok().json("No courses found for tutor".to_owned())
    } else {
        HttpResponse::Ok().json(filtered_courses)
    }
}

/// # Panics
///
/// Will panic if `courses` fails to lock.
pub async fn get_course_detail(
    app_state: web::Data<state::App>,
    params: web::Path<(i32, usize)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();

    let selected_course = app_state
        .courses
        .lock()
        .expect("failed to lock")
        .clone()
        .into_iter()
        .find(|x| x.tutor_id == tutor_id && x.course_id == Some(course_id))
        .ok_or("Course not found");

    selected_course.map_or_else(
        |_| HttpResponse::Ok().json("Course not found".to_owned()),
        |course| HttpResponse::Ok().json(course),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::{atomic::AtomicU32, Mutex};

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });
        let app_state: web::Data<state::App> = web::Data::new(state::App {
            health_check_response: String::new(),
            visit_count: AtomicU32::new(0),
            courses: Mutex::new(vec![]),
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state: web::Data<state::App> = web::Data::new(state::App {
            health_check_response: String::new(),
            visit_count: AtomicU32::new(0),
            courses: Mutex::new(vec![]),
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state: web::Data<state::App> = web::Data::new(state::App {
            health_check_response: String::new(),
            visit_count: AtomicU32::new(0),
            courses: Mutex::new(vec![]),
        });

        let params: web::Path<(i32, usize)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
