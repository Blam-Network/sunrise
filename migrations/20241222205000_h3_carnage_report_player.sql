-- Add migration script here
CREATE TABLE halo3.carnage_report_player (
    carnage_report_id UUID NOT NULL,  -- Reference to the carnage_report table
    player_identifier BIGINT NOT NULL,  -- 8 bytes (u64) for player_identifier
    player_index SMALLINT NOT NULL,
    machine_index SMALLINT,  -- Relates to machine_index in carnage_report_machine
    player_name VARCHAR(16) NOT NULL,  -- From player_configuration_from_client
    appearance_flags INT NOT NULL,  -- From player_configuration_from_client's appearance
    primary_color INT NOT NULL,  -- From player_configuration_from_client's appearance
    secondary_color INT NOT NULL,  -- From player_configuration_from_client's appearance
    tertiary_color INT NOT NULL,  -- From player_configuration_from_client's appearance
    player_model_choice INT NOT NULL,  -- From player_configuration_from_client's appearance
    foreground_emblem INT NOT NULL,  -- From player_configuration_from_client's appearance
    background_emblem INT NOT NULL,  -- From player_configuration_from_client's appearance
    emblem_flags INT NOT NULL,  -- From player_configuration_from_client's appearance
    emblem_primary_color INT NOT NULL,  -- From player_configuration_from_client's appearance
    emblem_secondary_color INT NOT NULL,  -- From player_configuration_from_client's appearance
    emblem_background_color INT NOT NULL,  -- From player_configuration_from_client's appearance
    spartan_model_area_0 INT NOT NULL,  -- From player_configuration_from_client's appearance
    spartan_model_area_1 INT NOT NULL,  -- From player_configuration_from_client's appearance
    spartan_model_area_2 INT NOT NULL,  -- From player_configuration_from_client's appearance
    spartan_model_area_3 INT NOT NULL,  -- From player_configuration_from_client's appearance
    elite_model_area_0 INT NOT NULL,  -- From player_configuration_from_client's appearance
    elite_model_area_1 INT NOT NULL,  -- From player_configuration_from_client's appearance
    elite_model_area_2 INT NOT NULL,  -- From player_configuration_from_client's appearance
    elite_model_area_3 INT NOT NULL,  -- From player_configuration_from_client's appearance
    service_tag VARCHAR(4) NOT NULL,  -- From player_configuration_from_client's appearance
    player_xuid BIGINT NOT NULL,  -- From player_configuration_from_client
    is_silver_or_gold_live BOOLEAN NOT NULL,  -- From player_configuration_from_client
    is_online_enabled BOOLEAN NOT NULL,  -- From player_configuration_from_client
    is_controller_attached BOOLEAN NOT NULL,  -- From player_configuration_from_client
    user_selected_team_index INT NOT NULL,  -- From player_configuration_from_client
    desires_veto BOOLEAN NOT NULL,  -- From player_configuration_from_client
    desires_rematch BOOLEAN NOT NULL,  -- From player_configuration_from_client
    hopper_access_flags INT NOT NULL,  -- From player_configuration_from_client
    is_free_live_gold_account BOOLEAN NOT NULL,  -- From player_configuration_from_client
    is_user_created_content_allowed BOOLEAN NOT NULL,  -- From player_configuration_from_client
    is_friend_created_content_allowed BOOLEAN NOT NULL,  -- From player_configuration_from_client
    is_griefer SMALLINT NOT NULL,  -- From player_configuration_from_client
    campaign_difficulty_completed INT NOT NULL,  -- From player_configuration_from_client
    bungienet_user_flags BIGINT NOT NULL,  -- From player_configuration_from_client
    gamer_region INT NOT NULL,  -- From player_configuration_from_client
    gamer_zone INT NOT NULL,  -- From player_configuration_from_client
    cheat_flags BIGINT NOT NULL,  -- From player_configuration_from_client
    ban_flags BIGINT NOT NULL,  -- From player_configuration_from_client
    repeated_play_coefficient INT NOT NULL,  -- From player_configuration_from_client
    experience_growth_banned BOOLEAN NOT NULL,  -- From player_configuration_from_client
    matchmade_ranked_games_played BIGINT NOT NULL,  -- From queried_player_statistics
    matchmade_ranked_games_completed BIGINT NOT NULL,  -- From queried_player_statistics
    matchmade_ranked_games_won BIGINT NOT NULL,  -- From queried_player_statistics
    matchmade_unranked_games_played BIGINT NOT NULL,  -- From queried_player_statistics
    matchmade_unranked_games_completed BIGINT NOT NULL,  -- From queried_player_statistics
    hopper_experience_base BIGINT NOT NULL,  -- From queried_player_statistics
    custom_games_completed BIGINT NOT NULL,  -- From queried_player_statistics
    hopper_experience_penalty BIGINT NOT NULL,  -- From queried_player_statistics
    first_played TIMESTAMP NOT NULL,  -- From queried_player_statistics
    last_played TIMESTAMP NOT NULL,  -- From queried_player_statistics
    global_statistics_valid SMALLINT NOT NULL,
    global_statistics_highest_skill BIGINT NOT NULL,
    global_statistics_experience_base BIGINT NOT NULL,
    global_statistics_experience_penalty BIGINT NOT NULL,
    hopper_statistics_valid SMALLINT NOT NULL,
    hopper_statistics_identifier INT NOT NULL,
    hopper_statistics_hopper_skill BIGINT NOT NULL,
    hopper_statistics_games_won BIGINT NOT NULL,
    hopper_statistics_games_played BIGINT NOT NULL,
    hopper_statistics_games_completed BIGINT NOT NULL,
    hopper_statistics_mu FLOAT,
    hopper_statistics_sigma FLOAT,
    player_team INT NOT NULL,  -- From player_configuration_from_host
    player_assigned_team INT NOT NULL,  -- From player_configuration_from_host
    host_stats_global_valid BOOLEAN NOT NULL,  -- From player_configuration_from_host
    host_stats_global_experience INT NOT NULL,  -- From player_configuration_from_host
    host_stats_global_rank INT NOT NULL,  -- From player_configuration_from_host
    host_stats_global_grade INT NOT NULL,  -- From player_configuration_from_host
    host_stats_hopper_valid BOOLEAN NOT NULL,  -- From player_configuration_from_host
    host_stats_hopper_skill INT NOT NULL,  -- From player_configuration_from_host
    host_stats_hopper_skill_display INT NOT NULL,  -- From player_configuration_from_host
    host_stats_hopper_skill_update_weight INT NOT NULL,  -- From player_configuration_from_host
    standing SMALLINT NOT NULL,  -- From s_blf_chunk_multiplayer_players_player
    result SMALLINT NOT NULL,  -- From s_blf_chunk_multiplayer_players_player
    score SMALLINT NOT NULL,  -- From s_blf_chunk_multiplayer_players_player
    PRIMARY KEY (carnage_report_id, player_index),
    FOREIGN KEY (carnage_report_id) REFERENCES halo3.carnage_report (id),
    FOREIGN KEY (machine_index, carnage_report_id) REFERENCES halo3.carnage_report_machine (machine_index, carnage_report_id)
);

-- Create index on player_xuid for optimized querying
CREATE INDEX idx_player_xuid ON halo3.carnage_report_player(player_xuid);

-- Create index on player_name for optimized querying
CREATE INDEX idx_player_name ON halo3.carnage_report_player(player_name);

ALTER TABLE halo3.carnage_report_matchmaking_options
    ADD CONSTRAINT pk_carnage_report_matchmaking_options PRIMARY KEY (id);