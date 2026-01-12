use crate::{errors::TutorError, models::Course};
use sqlx::postgres::PgPool;

// TODO: figure out why `tutor_id` can panic.
/// # Errors
///
/// Will return `Err` if course is not found in db.
#[allow(clippy::missing_panics_doc)]
pub async fn get_courses_for_tutor(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, TutorError> {
    let course_rows = sqlx::query!(
        r#"
        SELECT tutor_id, course_id, course_name, posted_time
        FROM course
        WHERE tutor_id = $1;
        "#,
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: course_row.posted_time,
        })
        .collect();

    match courses.len() {
        0 => Err(TutorError::NotFound(
            "courses not found for tutor".to_owned(),
        )),
        _ => Ok(courses),
    }
}

// TODO: figure out why `tutor_id` can panic.
/// # Errors
///
/// Will return `Err` if course or tutor is not found in db.
#[allow(clippy::missing_panics_doc)]
pub async fn get_course_details(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, TutorError> {
    let course_row = sqlx::query!(
        r#"
        SELECT tutor_id, course_id, course_name, posted_time
        FROM course
        WHERE tutor_id = $1 AND course_id = $2;
        "#,
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(course_row) = course_row {
        Ok(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: course_row.posted_time,
        })
    } else {
        Err(TutorError::NotFound("Course id not found".to_owned()))
    }
}

// TODO: figure out why `new_course` can panic.
/// # Errors
///
/// Will return `Err` if db fails to insert course.
#[allow(clippy::missing_panics_doc)]
pub async fn post_new_course(pool: &PgPool, new_course: Course) -> Result<Course, TutorError> {
    let course_row = sqlx::query!(
        r#"
        INSERT INTO course (course_id, tutor_id, course_name)
        VALUES ($1, $2, $3)
        RETURNING tutor_id, course_id, course_name, posted_time;
        "#,
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: course_row.posted_time,
    })
}
