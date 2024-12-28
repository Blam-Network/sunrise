-- Add migration script here
CREATE TABLE halo3.carnage_report_matchmaking_options (
  id UUID REFERENCES halo3.carnage_report(id) ON DELETE CASCADE, -- Foreign key relation to carnage_report
  hopper_identifier INT NOT NULL,
  hopper_name VARCHAR(32) NOT NULL,
  hopper_ranked BOOLEAN NOT NULL,
  hopper_team_based BOOLEAN NOT NULL,
  xlast_index INT NOT NULL,
  draw_probability BIGINT NOT NULL,
  beta FLOAT NOT NULL,
  tau FLOAT NOT NULL,
  experience_base_increment BIGINT NOT NULL,
  experience_penalty_decrement BIGINT NOT NULL,
  CONSTRAINT fk_carnage_report FOREIGN KEY (id) REFERENCES halo3.carnage_report(id) ON DELETE CASCADE
);