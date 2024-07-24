use crate::models::Course;
use std::sync::{atomic::AtomicU32, Mutex};

pub struct App {
    pub health_check_response: String,
    pub visit_count: AtomicU32,
    pub courses: Mutex<Vec<Course>>,
}
