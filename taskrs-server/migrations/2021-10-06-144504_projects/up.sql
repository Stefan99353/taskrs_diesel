-- Your SQL goes here

CREATE TABLE projects
(
    id          SERIAL PRIMARY KEY  NOT NULL,
    name        VARCHAR(256) UNIQUE NOT NULL,
    description TEXT,
    category_id INTEGER             NOT NULL,
    owner_id    INTEGER             NOT NULL,
    creator_id  INTEGER,
    updated_at  TIMESTAMP DEFAULT now(),
    created_at  TIMESTAMP DEFAULT now(),

    FOREIGN KEY (category_id) REFERENCES categories (id),
    FOREIGN KEY (owner_id) REFERENCES users (id),
    FOREIGN KEY (creator_id) REFERENCES users (id)
);

CREATE TABLE project_members
(
    project_id INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    is_admin   BOOLEAN NOT NULL,
    updated_at TIMESTAMP DEFAULT now(),
    created_at TIMESTAMP DEFAULT now(),

    PRIMARY KEY (project_id, user_id)
)