-- Add migration script here

CREATE TABLE auth_refresh_tokens
(
    id         SERIAL PRIMARY KEY NOT NULL,
    user_id    INTEGER            NOT NULL,
    token      VARCHAR(256)       NOT NULL,
    iat        BIGINT             NOT NULL,
    exp        BIGINT             NOT NULL,
    updated_at TIMESTAMP DEFAULT now(),
    created_at TIMESTAMP DEFAULT now(),

    FOREIGN KEY (user_id) REFERENCES users (id)
);
