CREATE TABLE students(
    id SERIAL PRIMARY KEY,
    student_id VARCHAR(255) NOT NULL,
    student_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    is_aide BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    token VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE user_login(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    login_time TIMESTAMP NOT NULL DEFAULT NOW(),
    logout_time TIMESTAMP NOT NULL DEFAULT NOW(),
    ip VARCHAR(255) NOT NULL,
    session_id VARCHAR(255) NOT NULL
);

CREATE TABLE visits(
    id SERIAL PRIMARY KEY,
    student_id INT NOT NULL,
    checked_in TIMESTAMP NOT NULL DEFAULT NOW(),
    left_at TIMESTAMP,
    constraint fk_student_id
        foreign key (student_id) 
        REFERENCES students(id)
);


--  Default Stuff
INSERT INTO users(username, token) VALUES ('admin', 'password');
INSERT INTO users(username, token) VALUES ('library', 'gobears');
INSERT INTO students(student_id, student_name, email, is_aide) VALUES ('0000001', 'Testing Student', 'tstudent1-00@spotsylvania.k12.va.us', false);
INSERT INTO students(student_id, student_name, email, is_aide) VALUES ('0000002', 'Testing Aide', 'tstudent2-00@spotsylvania.k12.va.us', true);