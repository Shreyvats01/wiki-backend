-- Add migration script here
ALTER TABLE todos
RENAME TO tasks;

ALTER TABLE tag_todo
RENAME TO tag_per_task;

ALTER TABLE tag_per_task
RENAME todo_id TO task_id;

ALTER TABLE daily_progress_todos
RENAME TO daily_tasks;

ALTER TABLE daily_tasks
RENAME todo_id TO task_id