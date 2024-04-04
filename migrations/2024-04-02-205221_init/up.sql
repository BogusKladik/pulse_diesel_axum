-- Your SQL goes here
CREATE TABLE users (
    id SERIAL NOT NULL,
    mail TEXT NOT NULL,
    hashed_password TEXT NOT NULL,

    CONSTRAINT pk_users PRIMARY KEY (id)
);

CREATE TABLE tokens (
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL,

    CONSTRAINT pk_tokens PRIMARY KEY (user_id),
    CONSTRAINT fk_tokens FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT user_id_unique UNIQUE (user_id)
);