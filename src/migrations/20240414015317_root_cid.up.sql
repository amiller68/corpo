CREATE TABLE root_cid (
    id SERIAL PRIMARY KEY,
    cid VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
