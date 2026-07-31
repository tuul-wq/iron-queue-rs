ALTER TABLE jobs
DROP COLUMN locked_by,
DROP COLUMN locked_at,
DROP COLUMN run_at,
DROP COLUMN last_error;
