\c lure_link;

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL NOT NULL,
    first_name varchar(255) NOT NULL,
    first_name_jp varchar(255) NOT NULL,
    last_name varchar(255) NOT NULL,
    last_name_jp varchar(255) NOT NULL,
    nick_name varchar(255) NOT NULL,

    PRIMARY KEY(id)
);