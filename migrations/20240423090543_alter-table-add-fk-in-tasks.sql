ALTER TABLE tasks ADD COLUMN task_list_id UUID;

ALTER TABLE tasks ADD CONSTRAINT fk_task_list_id FOREIGN KEY (task_list_id) REFERENCES tasklists(task_list_id) ON DELETE CASCADE;
