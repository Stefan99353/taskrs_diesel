CREATE TABLE users
(
    id         SERIAL PRIMARY KEY     NOT NULL,
    email      VARCHAR(100) UNIQUE    NOT NULL,
    password   VARCHAR(512)           NOT NULL,
    first_name VARCHAR(50),
    last_name  VARCHAR(50),
    activated  BOOLEAN   DEFAULT true NOT NULL,
    updated_at TIMESTAMP DEFAULT now(),
    created_at TIMESTAMP DEFAULT now()
);