ALTER TABLE jobs
DROP COLUMN locked_by,
DROP COLUMN locked_at,
DROP COLUMN run_at TIMESTAMPTZ,
DROP COLUMN last_error;

DROP INDEX jobs_claim_idx;
