CREATE TABLE tickets
(
    id uuid NOT NULL
        CONSTRAINT tickets_pk
            PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    updated TIMESTAMP WITH TIME ZONE,
    title TEXT NOT NULL,
    description TEXT,
    author_id uuid NOT NULL
        CONSTRAINT tickets_users_id_fk
            REFERENCES users
);

ALTER TABLE tickets
    OWNER TO kogasa;

CREATE INDEX tickets_author_id_created_index
    ON tickets (author_id, created);

CREATE INDEX tickets_created_index
    ON tickets (created DESC);

