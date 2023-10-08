\c lure_link;

CREATE TABLE IF NOT EXISTS prefectures (
    id BIGSERIAL NOT NULL,
    name varchar(255) NOT NULL,
    name_jp varchar(255) NOT NULL,
    PRIMARY KEY(id)
);