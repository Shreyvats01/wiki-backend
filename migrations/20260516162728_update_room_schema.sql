-- Add migration script here
ALTER TABLE rooms
RENAME COLUMN profile_pic TO profile_image;
