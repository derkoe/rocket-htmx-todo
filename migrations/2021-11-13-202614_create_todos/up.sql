-- Your SQL goes here
CREATE TABLE todos (
    id UUID NOT NULL,
    title VARCHAR(100) NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false,
    created_timestamp TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
);
