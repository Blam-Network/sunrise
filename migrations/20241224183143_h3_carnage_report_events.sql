-- Add migration script here
CREATE TABLE IF NOT EXISTS halo3.carnage_report_event_kill (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    carnage_report_id UUID,
    time BIGINT NOT NULL,
    killer_player_index SMALLINT NOT NULL,
    dead_player_index SMALLINT NOT NULL,
    killer_position FLOAT[3] NOT NULL,
    dead_position FLOAT[3] NOT NULL,
    kill_type INT NOT NULL,
    FOREIGN KEY (carnage_report_id)
        REFERENCES halo3.carnage_report (id)
        ON DELETE CASCADE,
    FOREIGN KEY (carnage_report_id, killer_player_index)
        REFERENCES halo3.carnage_report_player (carnage_report_id, player_index)
        ON DELETE CASCADE,
    FOREIGN KEY (carnage_report_id, dead_player_index)
        REFERENCES halo3.carnage_report_player (carnage_report_id, player_index)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS halo3.carnage_report_event_carry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    carnage_report_id UUID,
    time BIGINT NOT NULL,
    carry_player_index SMALLINT NOT NULL,
    position FLOAT[3] NOT NULL,
    weapon_index INT NOT NULL,
    carry_type INT NOT NULL,
    FOREIGN KEY (carnage_report_id)
        REFERENCES halo3.carnage_report (id)
        ON DELETE CASCADE,
    FOREIGN KEY (carnage_report_id, carry_player_index)
        REFERENCES halo3.carnage_report_player (carnage_report_id, player_index)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS halo3.carnage_report_event_score (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    carnage_report_id UUID,
    time BIGINT NOT NULL,
    score_player_index SMALLINT NOT NULL,
    position FLOAT[3] NOT NULL,
    weapon_index INT NOT NULL,
    score_type INT NOT NULL,
    FOREIGN KEY (carnage_report_id)
        REFERENCES halo3.carnage_report (id)
        ON DELETE CASCADE,
    FOREIGN KEY (carnage_report_id, score_player_index)
        REFERENCES halo3.carnage_report_player (carnage_report_id, player_index)
        ON DELETE CASCADE
);
