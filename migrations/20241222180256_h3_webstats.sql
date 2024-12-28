-- Add migration script here
CREATE SCHEMA IF NOT EXISTS halo3;

CREATE TABLE halo3.carnage_report (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  team_game BOOLEAN NOT NULL,
  game_id BIGINT NOT NULL,
  map_id INTEGER NOT NULL,
  scenario_path TEXT NOT NULL,
  map_variant_name VARCHAR(32) NOT NULL,
  game_variant_unique_id BIGINT NOT NULL,
  map_variant_unique_id BIGINT NOT NULL,
  started BOOLEAN NOT NULL,
  start_time TIMESTAMP NOT NULL,
  finished BOOLEAN NOT NULL,
  finish_time TIMESTAMP NOT NULL,
  migrated_to_group BOOLEAN NOT NULL,
  migrated_solo BOOLEAN NOT NULL,
  simulation_aborted BOOLEAN NOT NULL
);

-- Ensure reports are unique by checking the start and finish time, and game id.
CREATE UNIQUE INDEX unique_start_finish_game ON halo3.carnage_report (start_time, finish_time, game_id);