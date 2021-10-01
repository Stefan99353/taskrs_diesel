-- Add migration script here

CREATE TABLE permissions
(
    id          SERIAL PRIMARY KEY  NOT NULL,
    name        VARCHAR(128) UNIQUE NOT NULL,
    "group"     VARCHAR(128)        NOT NULL,
    description VARCHAR(512),
    updated_at  TIMESTAMP DEFAULT now(),
    created_at  TIMESTAMP DEFAULT now()
);

CREATE TABLE user_permissions
(
    user_id       INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
    updated_at    TIMESTAMP DEFAULT now(),
    created_at    TIMESTAMP DEFAULT now(),

    PRIMARY KEY (user_id, permission_id),

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (permission_id) REFERENCES permissions (id)
);
