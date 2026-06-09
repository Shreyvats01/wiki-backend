-- Add migration script here
ALTER TABLE tasks
ALTER COLUMN category_id DROP NOT NULL;
