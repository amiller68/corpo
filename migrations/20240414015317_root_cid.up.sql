CREATE TABLE root_cids (
    id SERIAL PRIMARY KEY NOT NULL,
    cid VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
