ALTER TABLE item RENAME TO todo;

ALTER TABLE todo RENAME COLUMN item_id TO todo_id;
