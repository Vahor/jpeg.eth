CREATE TABLE IF NOT EXISTS nft
(
    image_id VARCHAR(255) NOT NULL PRIMARY KEY,
    token_id VARCHAR(255) DEFAULT NULL,
    attributes VARCHAR(255) NOT NULL,

    UNIQUE (image_id)
);


