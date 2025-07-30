CREATE TYPE user_role AS ENUM ('admin', 'teacher', 'student');
CREATE TYPE question_type AS ENUM ('multiple_choice', 'essay', 'true_false');
CREATE TABLE IF NOT EXISTS users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT        NOT NULL,
    email       TEXT UNIQUE NOT NULL,
    password    TEXT        NOT NULL,
    is_active   BOOLEAN     DEFAULT TRUE,
    is_verified BOOLEAN     DEFAULT FALSE,
    role        user_role   NOT NULL,
    token       TEXT,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE IF NOT EXISTS classes (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    created_by UUID REFERENCES users (id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE IF NOT EXISTS class_members (
    user_id  UUID REFERENCES users (id),
    class_id UUID REFERENCES classes (id),
    PRIMARY KEY (user_id, class_id)
    );

CREATE TABLE IF NOT EXISTS exams (
    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title            TEXT      NOT NULL,
    description      TEXT,
    created_by       UUID REFERENCES users (id),
    duration_minutes INTEGER   NOT NULL,
    start_time       TIMESTAMPTZ NOT NULL,
    end_time         TIMESTAMPTZ NOT NULL,
    is_active        BOOLEAN     DEFAULT TRUE
    );

CREATE TABLE IF NOT EXISTS exam_assignments (
                                                exam_id  UUID REFERENCES exams (id),
    class_id UUID REFERENCES classes (id),
    PRIMARY KEY (exam_id, class_id)
    );

CREATE TABLE IF NOT EXISTS questions (
                                         id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exam_id        UUID REFERENCES exams (id) ON DELETE CASCADE,
    question_text  TEXT          NOT NULL,
    question_type  question_type NOT NULL,
    correct_answer TEXT,
    score          INTEGER       DEFAULT 1
    );

CREATE TABLE IF NOT EXISTS choices (
                                       id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_id UUID REFERENCES questions (id) ON DELETE CASCADE,
    choice_text TEXT NOT NULL,
    is_correct  BOOLEAN DEFAULT FALSE
    );

CREATE TABLE IF NOT EXISTS exam_attempts (
                                             id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID REFERENCES users (id),
    exam_id      UUID REFERENCES exams (id),
    started_at   TIMESTAMPTZ,
    submitted_at TIMESTAMPTZ,
    score_total  INTEGER
    );

CREATE TABLE IF NOT EXISTS answers (
                                       id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    attempt_id    UUID REFERENCES exam_attempts (id) ON DELETE CASCADE,
    question_id   UUID REFERENCES questions (id),
    answer_text   TEXT,
    is_correct    BOOLEAN,
    score_awarded INTEGER
    );


);


ALTER TABLE users
ALTER COLUMN created_at TYPE TIMESTAMPTZ,
ALTER COLUMN updated_at TYPE TIMESTAMPTZ;

ALTER TABLE classes
ALTER COLUMN created_at TYPE TIMESTAMPTZ;

ALTER TABLE exams
ALTER COLUMN start_time TYPE TIMESTAMPTZ,
ALTER COLUMN end_time TYPE TIMESTAMPTZ;

ALTER TABLE exam_attempts
ALTER COLUMN started_at TYPE TIMESTAMPTZ,
ALTER COLUMN submitted_at TYPE TIMESTAMPTZ;