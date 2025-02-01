CREATE OR REPLACE PROCEDURE halo3.unlock_recon(IN player_xuid_in bigint)
 LANGUAGE plpgsql
AS $procedure$
BEGIN
    -- Check if player data exists
    IF NOT EXISTS (SELECT 1 FROM halo3.player_data WHERE player_xuid = player_xuid_in) THEN
        -- If not, create a new record with the default values
        INSERT INTO halo3.player_data (player_xuid, road_to_recon_completed)
        VALUES (player_xuid_in, true);
ELSE

    -- If it exists, update the highest_skill
UPDATE halo3.player_data
SET road_to_recon_completed = true
WHERE player_xuid = player_xuid_in;
END IF;
END;
$procedure$
;
