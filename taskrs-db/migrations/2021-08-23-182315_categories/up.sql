-- Your SQL goes here

CREATE TABLE categories
(
    id                 SERIAL PRIMARY KEY  NOT NULL,
    name               VARCHAR(100) UNIQUE NOT NULL,
    parent_category_id INTEGER,
    updated_at         TIMESTAMP DEFAULT now(),
    created_at         TIMESTAMP DEFAULT now(),

    FOREIGN KEY (parent_category_id) REFERENCES categories (id)
);
