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
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, class_id)
    );

CREATE TABLE IF NOT EXISTS exams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title            TEXT      NOT NULL,
    description      TEXT,
    created_by       UUID REFERENCES users (id),
    duration_minutes INTEGER   NOT NULL,
    start_time       TIMESTAMPTZ NOT NULL,
    end_time         TIMESTAMPTZ NOT NULL,
    is_active        BOOLEAN     DEFAULT TRUE,
    category         TEXT        NOT NULL,
    difficulty     TEXT        NOT NULL
    );

CREATE TABLE IF NOT EXISTS exam_assignments (
    exam_id  UUID REFERENCES exams (id),
    class_id UUID REFERENCES classes (id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
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
    is_correct  BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
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





CREATE INDEX idx_class_members_user_id ON class_members(user_id);
CREATE INDEX idx_class_members_class_id ON class_members(class_id);
CREATE INDEX idx_questions_exam_id ON questions(exam_id);
CREATE INDEX idx_choices_question_id ON choices(question_id);
CREATE INDEX idx_exam_assignments_exam_id ON exam_assignments(exam_id);
CREATE INDEX idx_exam_assignments_class_id ON exam_assignments(class_id);
CREATE INDEX idx_exam_attempts_user_id ON exam_attempts(user_id);
