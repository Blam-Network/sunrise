CREATE OR REPLACE FUNCTION get_carnage_report_details(c_report_id UUID)
RETURNS TEXT AS $$
DECLARE
result TEXT;
BEGIN
SELECT
    row_to_json(cr)::jsonb || jsonb_build_object(
            'map_variant_unique_id', to_jsonb(to_hex(cr.map_variant_unique_id)),
            'game_variant_unique_id', to_jsonb(to_hex(cr.game_variant_unique_id)),
            'matchmaking_options', (
                SELECT row_to_json(mmo)::jsonb
                FROM halo3.carnage_report_matchmaking_options mmo
                WHERE mmo.id = cr.id
                LIMIT 1
            ),
            'teams', (
                SELECT jsonb_agg(
                    jsonb_set(
                        row_to_json(team)::jsonb,
                        '{statistics}',
                        to_jsonb(stats)
                    )
                )
                FROM halo3.carnage_report_team team
                JOIN halo3.carnage_report_team_statistics stats
                    ON stats.carnage_report_id = team.carnage_report_id
                    AND stats.team_index = team.team_index
                WHERE team.carnage_report_id = cr.id
            ),
            'players', (
                SELECT jsonb_agg(
                    jsonb_set(
                        jsonb_set(
                            row_to_json(player)::jsonb,
                            '{player_xuid}',
                            to_jsonb(to_hex(player.player_xuid)),
                            false
                        ),
                        '{player_identifier}',
                        to_jsonb(to_hex(player.player_identifier)),
                        false
                    ) || jsonb_build_object(
                        'statistics', to_jsonb(stats),
                        'achievements', to_jsonb(achievements),
                        'medals', to_jsonb(medals),
                        'damage_statistics', (
                            SELECT jsonb_agg(
                                jsonb_build_object(
                                    'damage_source', ds.damage_source,
                                    'kills', ds.kills,
                                    'deaths', ds.deaths,
                                    'betrayals', ds.betrayals,
                                    'suicides', ds.suicides,
                                    'headshots', ds.headshots
                                )
                            )
                            FROM halo3.carnage_report_player_damage_statistics ds
                            WHERE ds.carnage_report_id = player.carnage_report_id
                              AND ds.player_index = player.player_index
                        ),
                        'machine_host', m.machine_host,
                        'machine_initial_host', m.machine_initial_host,
                        'session_party_nonce', to_jsonb(to_hex(m.session_party_nonce))
                    )
                )
                FROM halo3.carnage_report_player player
                LEFT JOIN halo3.carnage_report_player_statistics stats
                    ON stats.carnage_report_id = player.carnage_report_id
                    AND stats.player_index = player.player_index
                LEFT JOIN halo3.carnage_report_player_achievements achievements
                    ON achievements.carnage_report_id = player.carnage_report_id
                    AND achievements.player_index = player.player_index
                LEFT JOIN halo3.carnage_report_player_medals medals
                    ON medals.carnage_report_id = player.carnage_report_id
                    AND medals.player_index = player.player_index
                LEFT JOIN halo3.carnage_report_machine m
                    ON m.carnage_report_id = player.carnage_report_id
                    AND m.machine_index = player.machine_index
                WHERE player.carnage_report_id = cr.id
            ),
            'events', jsonb_build_object(
                'kill_events', (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'time', ke.time,
                            'killer_player_index', ke.killer_player_index,
                            'dead_player_index', ke.dead_player_index,
                            'killer_position', ke.killer_position,
                            'dead_position', ke.dead_position,
                            'kill_type', ke.kill_type
                        )
                    )
                    FROM halo3.carnage_report_event_kill ke
                    WHERE ke.carnage_report_id = cr.id
                ),
                'carry_events', (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'time', ce.time,
                            'carry_player_index', ce.carry_player_index,
                            'position', ce.position,
                            'weapon_index', ce.weapon_index,
                            'carry_type', ce.carry_type
                        )
                    )
                    FROM halo3.carnage_report_event_carry ce
                    WHERE ce.carnage_report_id = cr.id
                ),
                'score_events', (
                    SELECT jsonb_agg(
                        jsonb_build_object(
                            'time', se.time,
                            'score_player_index', se.score_player_index,
                            'position', se.position,
                            'weapon_index', se.weapon_index,
                            'score_type', se.score_type
                        )
                    )
                    FROM halo3.carnage_report_event_score se
                    WHERE se.carnage_report_id = cr.id
                )
            ),
            'game_variant', (
                SELECT row_to_json(gv)::jsonb || jsonb_build_object(
                    'unique_id', to_jsonb(to_hex(gv.unique_id)),
                    'author_id', to_jsonb(to_hex(gv.author_id))
                )
                FROM halo3.carnage_report_game_variant gv
                WHERE gv.id = cr.id
                LIMIT 1
            ),
            'player_interactions', (
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'left_player_index', pi.left_player_index,
                        'right_player_index', pi.right_player_index,
                        'killed', pi.killed,
                        'killed_by', pi.killed_by
                    )
                )
                FROM halo3.carnage_report_player_interaction pi
                WHERE pi.carnage_report_id = cr.id
            )
        ) AS reports_with_details
INTO result
FROM halo3.carnage_report cr
WHERE cr.id = c_report_id;

RETURN result::TEXT;
END;
$$ LANGUAGE plpgsql;