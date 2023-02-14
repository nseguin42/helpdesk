CREATE TABLE comments
(
    id uuid NOT NULL
        CONSTRAINT comments_pk
            PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    text TEXT NOT NULL,
    author_id uuid NOT NULL
        CONSTRAINT comments_users_id_fk
            REFERENCES users,
    ticket_id uuid NOT NULL
        CONSTRAINT comments_tickets_id_fk
            REFERENCES tickets
);

ALTER TABLE comments
    OWNER TO kogasa;

CREATE INDEX comments_created_index
    ON comments (created DESC);

CREATE INDEX comments_ticket_id_created_index
    ON comments (ticket_id, created);

