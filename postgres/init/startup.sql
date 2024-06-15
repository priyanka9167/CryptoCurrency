
-- CREATE DATABASE IF NOT EXISTS crypto;

-- CREATE TABLE IF NOT EXISTS bitcoin_data (
--     id SERIAL PRIMARY KEY,
--     name VARCHAR(50),
--     bitcoin_height INT,
--     timestamp INT
-- );


CREATE TABLE IF NOT EXISTS blocks  (
    id SERIAL PRIMARY KEY,
    blockchain VARCHAR(255) NOT NULL,
    block_number BIGINT NOT NULL,
    total_transactions BIGINT NOT NULL,
    gas_used VARCHAR(255) NOT NULL,
    miner VARCHAR(255) NOT NULL,
    block_time TIMESTAMP NOT NULL,
    difficulty VARCHAR(255) NOT NULL,
    CONSTRAINT unique_block_number UNIQUE (blockchain, block_number)
);

CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    block_id INT REFERENCES blocks(id) ON DELETE CASCADE,
    transaction_hash VARCHAR(255) NOT NULL,
    transaction_time TIMESTAMP NOT NULL,
    from_address VARCHAR(255),
    to_address VARCHAR(255),
    value VARCHAR(255),
    gas VARCHAR(255),
    gas_price VARCHAR(255),
    CONSTRAINT unique_transaction_hash UNIQUE (block_id, transaction_hash)
);


