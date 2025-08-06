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



CREATE TYPE course_type AS ENUM ('course', 'exam', 'assignment', 'quiz', 'project');
CREATE TYPE progress_status AS ENUM ('started', 'inprogress', 'completed', 'failed', 'enrolled');

-- Create user_progress table
CREATE TABLE IF NOT EXISTS user_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_name VARCHAR(255) NOT NULL,
    course_type course_type NOT NULL,
    progress_percentage INTEGER CHECK (progress_percentage >= 0 AND progress_percentage <= 100),
    status progress_status NOT NULL DEFAULT 'started',
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    total_score INTEGER,
    max_score INTEGER,
    level INTEGER NOT NULL DEFAULT 1,
    experience_points INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create user_levels table
CREATE TABLE IF NOT EXISTS user_levels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    current_level INTEGER NOT NULL DEFAULT 1,
    total_experience INTEGER NOT NULL DEFAULT 0,
    experience_to_next_level INTEGER NOT NULL DEFAULT 100,
    level_title VARCHAR(50) NOT NULL DEFAULT 'Beginner',
    achievements JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_user_progress_user_id ON user_progress(user_id);
CREATE INDEX idx_user_progress_status ON user_progress(status);
CREATE INDEX idx_user_progress_course_type ON user_progress(course_type);
CREATE INDEX idx_user_progress_started_at ON user_progress(started_at);
CREATE INDEX idx_user_levels_user_id ON user_levels(user_id);
CREATE INDEX idx_user_levels_level_exp ON user_levels(current_level, total_experience);

-- Add function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for auto-updating updated_at
CREATE TRIGGER update_user_progress_updated_at 
    BEFORE UPDATE ON user_progress 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_levels_updated_at 
    BEFORE UPDATE ON user_levels 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Initialize user_levels for existing users (optional)
-- Remove this section if you don't want to initialize existing users
INSERT INTO user_levels (user_id, current_level, total_experience, experience_to_next_level, level_title, achievements)
SELECT 
    id as user_id,
    1 as current_level,
    0 as total_experience,
    100 as experience_to_next_level,
    'Beginner' as level_title,
    '[]'::jsonb as achievements
FROM users 
ON CONFLICT (user_id) DO NOTHING;


CREATE INDEX idx_class_members_user_id ON class_members(user_id);
CREATE INDEX idx_class_members_class_id ON class_members(class_id);
CREATE INDEX idx_questions_exam_id ON questions(exam_id);
CREATE INDEX idx_choices_question_id ON choices(question_id);
CREATE INDEX idx_exam_assignments_exam_id ON exam_assignments(exam_id);
CREATE INDEX idx_exam_assignments_class_id ON exam_assignments(class_id);
CREATE INDEX idx_exam_attempts_user_id ON exam_attempts(user_id);
