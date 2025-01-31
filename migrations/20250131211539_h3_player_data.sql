CREATE TABLE halo3.player_data (
   player_xuid BIGINT PRIMARY KEY,  -- u64 equivalent in Postgres
   hopper_access INT DEFAULT 0,
   highest_skill INT DEFAULT 0,
   road_to_recon_completed BOOLEAN DEFAULT FALSE,
   is_bungie BOOLEAN DEFAULT FALSE,
   is_pro BOOLEAN DEFAULT FALSE,
   has_recon BOOLEAN DEFAULT FALSE,
   hopper_directory_override VARCHAR(32) DEFAULT NULL  -- Updated to have a max length of 32 characters
);

-- DROP FUNCTION halo3.get_or_create_player_data(int8);

CREATE OR REPLACE FUNCTION halo3.get_or_create_player_data(player_xuid_in bigint)
 RETURNS TABLE(player_xuid bigint, hopper_access integer, highest_skill integer, road_to_recon_completed boolean, is_bungie boolean, is_pro boolean, has_recon boolean, hopper_directory_override character varying)
 LANGUAGE plpgsql
AS $function$
BEGIN
    -- Check if player data exists
    IF NOT EXISTS (SELECT 1 FROM halo3.player_data pd WHERE pd.player_xuid = player_xuid_in) THEN
        -- If not, create a new record with default values
        INSERT INTO halo3.player_data (player_xuid)
        VALUES (player_xuid_in);
END IF;

    -- Return the player data (if it exists or after creating it)
RETURN QUERY
SELECT
    pd.player_xuid,
    pd.hopper_access,
    pd.highest_skill,
    pd.road_to_recon_completed,
    pd.is_bungie,
    pd.is_pro,
    pd.has_recon,
    pd.hopper_directory_override
FROM halo3.player_data pd
WHERE pd.player_xuid = player_xuid_in;
END;
$function$
;


CREATE OR REPLACE PROCEDURE halo3.update_highest_skill(IN player_xuid BIGINT, IN new_highest_skill INT)
LANGUAGE plpgsql
AS $$
BEGIN
    -- Check if player data exists
    IF NOT EXISTS (SELECT 1 FROM halo3.player_data WHERE player_xuid = player_xuid) THEN
        -- If not, create a new record with the default values
        INSERT INTO halo3.player_data (player_xuid, highest_skill)
        VALUES (player_xuid, new_highest_skill);
ELSE
        -- If it exists, update the highest_skill
UPDATE halo3.player_data
SET highest_skill = new_highest_skill
WHERE player_xuid = player_xuid;
END IF;
END;
$$;
