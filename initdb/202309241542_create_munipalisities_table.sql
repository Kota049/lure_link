\c lure_link;

CREATE TABLE IF NOT EXISTS municipalities (
    id BIGSERIAL NOT NULL,
    prefecture_id BIGINT NOT NULL,
    name varchar(255) NOT NULL,
    name_jp varchar(255) NOT NULL,

    PRIMARY KEY(id),
    FOREIGN KEY(prefecture_id) REFERENCES prefectures(id)
);