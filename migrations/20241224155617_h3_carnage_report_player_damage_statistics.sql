-- Add migration script here
CREATE TABLE halo3.carnage_report_player_damage_statistics (
    carnage_report_id UUID NOT NULL,
    player_index SMALLINT NOT NULL,
    damage_source VARCHAR NOT NULL,
    kills SMALLINT NOT NULL,
    deaths SMALLINT NOT NULL,
    betrayals SMALLINT NOT NULL,
    suicides SMALLINT NOT NULL,
    headshots SMALLINT NOT NULL,
    PRIMARY KEY (carnage_report_id, player_index, damage_source),
    CONSTRAINT fk_damage_statistics_carnage_report_player FOREIGN KEY (carnage_report_id, player_index)
        REFERENCES halo3.carnage_report_player (carnage_report_id, player_index)
        ON DELETE CASCADE
);
