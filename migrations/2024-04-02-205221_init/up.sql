-- Your SQL goes here
CREATE TABLE users (
    id SERIAL NOT NULL,
    mail TEXT NOT NULL,
    hashed_password TEXT NOT NULL,

    CONSTRAINT pk_users PRIMARY KEY (id)
);

CREATE TABLE tokens (
    id SERIAL NOT NULL,
    user_id SERIAL NOT NULL,
    token TEXT NOT NULL,

    CONSTRAINT pk_tokens PRIMARY KEY (id),
    CONSTRAINT fk_tokens FOREIGN KEY (user_id) REFERENCES users (id)
);