\c lure_link;

CREATE TABLE IF NOT EXISTS recruitments (
    id BIGSERIAL NOT NULL,
    rendezvous_prefecture_id BIGINT NOT NULL,
    rendezvous_municipality_id BIGINT NOT NULL,
    rendezvous_point varchar(255) NOT NULL,
    destination_prefecture_id BIGINT NOT NULL,
    destination_municipality_id BIGINT NOT NULL,
    destination_point varchar(255) NOT NULL,
    organizer_id BIGINT NOT NULL,
    participant_count SMALLINT NOT NULL,
    start_datetime timestamp NOT NULL,
    application_datetime timestamp NOT NULL,
    budget INT NOT NULL,

    PRIMARY KEY(id),
    FOREIGN KEY (rendezvous_prefecture_id) REFERENCES prefectures(id),
    FOREIGN KEY (rendezvous_municipality_id) REFERENCES municipalities(id),
    FOREIGN KEY (destination_prefecture_id) REFERENCES prefectures(id),
    FOREIGN KEY (destination_municipality_id) REFERENCES municipalities(id),
    FOREIGN KEY (organizer_id) REFERENCES users(id)
);