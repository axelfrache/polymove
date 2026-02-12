CREATE TABLE IF NOT EXISTS students (
    id UUID PRIMARY KEY,
    firstname TEXT NOT NULL,
    name TEXT NOT NULL,
    domain TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_students_domain ON students(domain);
