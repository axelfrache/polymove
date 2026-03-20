CREATE TABLE IF NOT EXISTS internships (
    id UUID PRIMARY KEY,
    student_id UUID NOT NULL REFERENCES students(id),
    offer_id TEXT NOT NULL,
    approved BOOLEAN NOT NULL,
    message TEXT NOT NULL
);
