-- Add migration script here
CREATE TABLE halo3.carnage_report_team (
    team_index INT NOT NULL, -- Index of the team
    carnage_report_id UUID NOT NULL REFERENCES halo3.carnage_report(id) ON DELETE CASCADE, -- Foreign key to carnage_report
    standing SMALLINT NOT NULL, -- Team's standing
    score SMALLINT NOT NULL, -- Team's score
    PRIMARY KEY (team_index, carnage_report_id) -- Compound primary key
);