ALTER TABLE halo3.carnage_report_machine ADD inaddr text NULL;
ALTER TABLE halo3.carnage_report_machine ADD inaddr_online text NULL;

-- backfill
UPDATE halo3.carnage_report_machine
SET
    inaddr = host(inet '0.0.0.0' + ('x' || lpad(encode(substring(session_secure_address FROM 1 FOR 4), 'hex'), 8, '0'))::bit(32)::bigint),
    inaddr_online = host(inet '0.0.0.0' + ('x' || lpad(encode(substring(session_secure_address FROM 5 FOR 4), 'hex'), 8, '0'))::bit(32)::bigint),
    session_secure_address = substring(session_secure_address FROM 9)
WHERE length(session_secure_address) = 36;
RETURNING inaddr, inaddr_online, encode(session_secure_address, 'hex');