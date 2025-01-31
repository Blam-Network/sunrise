CREATE SCHEMA IF NOT EXISTS odst;


-- Drop table if it exists
DROP TABLE IF EXISTS odst.blind_screenshot;

-- Create table
CREATE TABLE odst.blind_screenshot (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    unique_id BIGINT,
    name VARCHAR(16) NOT NULL,
    description VARCHAR(128) NOT NULL,
    author VARCHAR(16) NOT NULL,
    file_type INTEGER NOT NULL,
    author_is_xuid_online BOOLEAN NOT NULL,
    author_id BIGINT NOT NULL,
    size_in_bytes BIGINT NOT NULL,
    date TIMESTAMP NOT NULL,
    length_seconds INTEGER NOT NULL,
    campaign_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    game_engine_type INTEGER NOT NULL,
    campaign_difficulty INTEGER NOT NULL,
    campaign_insertion_point SMALLINT NOT NULL,
    campaign_survival_enabled BOOLEAN NOT NULL,
    game_id BIGINT NOT NULL,

    -- screenshot header
    game_tick INTEGER NOT NULL,
    film_tick INTEGER NOT NULL,
    jpeg_length INTEGER NOT NULL,
    camera_position FLOAT[3] NOT NULL,
    pixel_width SMALLINT NOT NULL,
    pixel_height SMALLINT NOT null,

    CONSTRAINT unique_unique_id_date UNIQUE (unique_id, date, game_id)
);