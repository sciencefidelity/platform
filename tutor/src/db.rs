use crate::models::Course;
use sqlx::postgres::PgPool;

/// # Panics
///
/// Will panic if db query fails.
pub async fn get_courses_for_tutor(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM course where tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await
    .expect("db query failed");

    course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: course_row.posted_time,
        })
        .collect()
}

/// # Panics
///
/// Will panic if db query fails.
pub async fn get_course_details(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM course where tutor_id = $1 and course_id = $2",
        tutor_id, course_id
    ).fetch_one(pool)
        .await
        .expect("db query failed");

    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: course_row.posted_time,
    }
}

/// # Panics
///
/// Will panic if db query fails.
pub async fn post_new_course(pool: &PgPool, new_course: Course) -> Course {
    let course_row = sqlx::query!(
        "insert into course (course_id, tutor_id, course_name)
        values ($1, $2, $3) 
        returning tutor_id, course_id, course_name, posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await
    .expect("db query failed");

    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: course_row.posted_time,
    }
}
