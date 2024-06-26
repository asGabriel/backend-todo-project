CREATE TABLE IF NOT EXISTS TASK (
    task_id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    status BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
