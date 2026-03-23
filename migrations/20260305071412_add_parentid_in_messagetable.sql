ALTER TABLE user_messages
ADD COLUMN parent_id UUID NULL;

ALTER TABLE user_messages
ADD CONSTRAINT messages_parent_fk
    FOREIGN KEY (parent_id)
    REFERENCES user_messages(id)
    ON DELETE SET NULL;
