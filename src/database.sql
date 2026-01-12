DROP TABLE IF EXISTS course;
CREATE TABLE course
(
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    posted_time TIMESTAMP DEFAULT NOW()
);

/* Load seed data for testing */
INSERT INTO course (course_id, tutor_id, course_name, posted_time)
VALUES (1, 1, 'First course', '2024-07-17 05:40:00');
INSERT INTO course (course_id, tutor_id, course_name, posted_time)
VALUES (2, 1, 'Second course', '2024-07-18 05:45:00');
