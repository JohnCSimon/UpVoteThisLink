-- init.sql
CREATE TABLE hashedurls (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO hashedurls (url, hash) VALUES ('https://example.com', 'examplehash'), ('https://yahoo.com', '0xdeadbeef');
