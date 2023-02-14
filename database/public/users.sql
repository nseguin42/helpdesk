CREATE TABLE users
(
    id uuid NOT NULL
        CONSTRAINT users_pk
            PRIMARY KEY,
    name TEXT NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL
);

ALTER TABLE users
    OWNER TO kogasa;

CREATE INDEX users_created_index
    ON users (created);

