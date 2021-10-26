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