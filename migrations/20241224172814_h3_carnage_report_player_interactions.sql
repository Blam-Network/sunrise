-- Add migration script here
CREATE TABLE IF NOT EXISTS halo3.carnage_report_player_interaction (
    carnage_report_id UUID NOT NULL,
    left_player_index SMALLINT NOT NULL,
    right_player_index SMALLINT NOT NULL,
    killed SMALLINT NOT NULL,
    killed_by SMALLINT NOT NULL,
    PRIMARY KEY (carnage_report_id, left_player_index, right_player_index),
    FOREIGN KEY (left_player_index, carnage_report_id)
        REFERENCES halo3.carnage_report_player (player_index, carnage_report_id)
        ON DELETE CASCADE,
    FOREIGN KEY (right_player_index, carnage_report_id)
        REFERENCES halo3.carnage_report_player (player_index, carnage_report_id)
        ON DELETE CASCADE
);