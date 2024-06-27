
-- CREATE DATABASE IF NOT EXISTS crypto;

-- CREATE TABLE IF NOT EXISTS bitcoin_data (
--     id SERIAL PRIMARY KEY,
--     name VARCHAR(50),
--     bitcoin_height INT,
--     timestamp INT
-- );


CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
    block_hash VARCHAR(255) NOT NULL,
    block_height BIGINT NOT NULL,
    total_transaction INT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    transaction_in_usd DOUBLE PRECISION NOT NULL,
    CONSTRAINT unique_block_hash UNIQUE (block_hash)
);



