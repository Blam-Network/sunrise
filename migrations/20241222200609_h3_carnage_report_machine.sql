-- Add migration script here
CREATE TABLE halo3.carnage_report_machine (
    carnage_report_id UUID NOT NULL,
    machine_index SMALLINT NOT NULL,
    machine_identifier BYTEA NOT NULL,  -- Assuming the machine identifier is a 6-byte array, stored as BIGINT for simplicity.
    -- Fields for machine_data (s_game_results_machine_data)
    machine_exists BOOLEAN,  -- Maps to `exists` (s_bool)
    machine_connected_to_host BOOLEAN,  -- Maps to `connected_to_host` (s_bool)
    machine_host BOOLEAN,  -- Maps to `host` (s_bool)
    machine_initial_host BOOLEAN,  -- Maps to `initial_host` (s_bool)
    machine_voluntary_quit BOOLEAN,  -- Maps to `voluntary_quit` (s_bool)
    machine_bandwidth_events_0 INT,  -- Maps to `bandwidth_events[0]`
    machine_bandwidth_events_1 INT,  -- Maps to `bandwidth_events[1]`
    machine_bandwidth_events_2 INT,  -- Maps to `bandwidth_events[2]`
    machine_bandwidth_events_3 INT,  -- Maps to `bandwidth_events[3]`
    machine_bandwidth_events_4 INT,  -- Maps to `bandwidth_events[4]`
    -- Fields for session_info (s_machine_session_info)
    session_exists BOOLEAN,  -- Maps to `exists` (s_bool)
    session_has_hard_drive BOOLEAN,  -- Maps to `has_hard_drive` (s_bool)
    session_party_nonce BIGINT,  -- Maps to `party_nonce` (u64)
    session_secure_address BYTEA,  -- Maps to `secure_address` (StaticArray<u8, 36>)
    session_network_version_number INT,  -- Maps to `network_version_number` (i32)
    session_peer_estimated_downstream_bandwidth_bps INT,  -- Maps to `peer_estimated_downstream_bandwidth_bps` (i32)
    session_peer_estimated_upstream_bandwidth_bps INT,  -- Maps to `peer_estimated_upstream_bandwidth_bps` (i32)
    session_peer_nat_type INT,  -- Maps to `peer_nat_type` (e_online_nat_type, stored as i32)
    session_peer_to_peer_connectivity_mask INT,  -- Maps to `peer_to_peer_connectivity_mask` (u16)
    session_peer_to_peer_probed_mask INT,  -- Maps to `peer_to_peer_probed_mask` (u16)
    PRIMARY KEY (carnage_report_id, machine_index),
    CONSTRAINT fk_carnage_report FOREIGN KEY (carnage_report_id) REFERENCES halo3.carnage_report (id) ON DELETE CASCADE
);

ALTER TABLE halo3.carnage_report
    ADD COLUMN in_group_session BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN in_squad_session BOOLEAN NOT NULL DEFAULT FALSE;