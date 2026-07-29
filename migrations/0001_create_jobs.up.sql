CREATE TYPE job_status AS ENUM (
    'queued',
    'running',
    'completed',
    'failed',
    'cancelled',
    'dead'
);

-- Enum order is low < normal < high, so DESC returns high-priority jobs first
CREATE TYPE job_priority AS ENUM (
    'low',
    'normal',
    'high'
);

CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    name TEXT NOT NULL,
    payload JSONB NOT NULL,
    status job_status NOT NULL,

    priority job_priority NOT NULL,
    retry_count SMALLINT NOT NULL DEFAULT 0,
    max_retries SMALLINT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT jobs_name_not_empty CHECK (length(trim(name)) > 0),
    CONSTRAINT jobs_retry_count_non_negative CHECK (retry_count >= 0),
    CONSTRAINT jobs_max_retries_non_negative CHECK (max_retries >= 0),
    CONSTRAINT jobs_retry_count_within_limit CHECK (retry_count <= max_retries)
);
