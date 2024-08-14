-- Your SQL goes here
CREATE TABLE priorities (
    priority_id SERIAL PRIMARY KEY,
    priority_name VARCHAR(255) NOT NULL
);

CREATE TABLE tags (
    tags_id SERIAL PRIMARY KEY,
    tag_name VARCHAR(255) NOT NULL
);

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    due_date DATE,
    priority_id INT,
    tags_id INT,
    created_by INT,
    assigned_to INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (priority_id) REFERENCES priorities(priority_id),
    FOREIGN KEY (tags_id) REFERENCES tags(tags_id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (assigned_to) REFERENCES users(id)
);