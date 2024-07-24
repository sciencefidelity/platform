use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: Option<usize>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Self>> for Course {
    fn from(course: web::Json<Self>) -> Self {
        Self {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}
