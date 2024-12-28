-- Add migration script here
CREATE TABLE halo3.carnage_report_game_variant (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    game_engine SMALLINT NOT NULL,
    unique_id BIGINT NOT NULL,
    name VARCHAR(16) NOT NULL,
    description VARCHAR(128) NOT NULL,
    author VARCHAR(16) NOT NULL,
    file_type INTEGER NOT NULL,
    author_is_xuid_online BOOLEAN NOT NULL,
    author_id BIGINT NOT NULL,
    size_in_bytes BIGINT NOT NULL,
    date TIMESTAMP NOT NULL,
    CONSTRAINT fk_carnage_report FOREIGN KEY (id) REFERENCES halo3.carnage_report(id) ON DELETE CASCADE
);
