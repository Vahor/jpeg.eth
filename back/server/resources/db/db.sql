CREATE TABLE IF NOT EXISTS images
(
    image_id INT          NOT NULL PRIMARY KEY,
    url      VARCHAR(255) NOT NULL
    );

CREATE TABLE IF NOT EXISTS nft
(
    nft_id   INT          NOT NULL PRIMARY KEY,
    image_id INT          NOT NULL,
    FOREIGN KEY (image_id) REFERENCES IMAGE (image_id)
);
