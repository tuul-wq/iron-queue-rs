CREATE TYPE job_status AS ENUM (
    'queued',
    'running',
    'succeeded',
    'failed',
    'dead',
    'cancelled'
);

-- Enum order is low < normal < high, so DESC returns high-priority jobs first
CREATE TYPE job_priority AS ENUM (
    'low',
    'normal',
    'high'
);

CREATE TABLE jobs (
    id UUID PRIMARY KEY,
    kind TEXT NOT NULL,
    payload JSONB NOT NULL,
    status job_status NOT NULL DEFAULT 'queued',

    priority job_priority NOT NULL DEFAULT 'normal',
    attempts INT NOT NULL DEFAULT 0,
    max_retries INT NOT NULL DEFAULT 3,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT jobs_kind_not_empty CHECK (length(trim(kind)) > 0),
    CONSTRAINT jobs_attempts_non_negative CHECK (attempts >= 0),
    CONSTRAINT jobs_max_retries_non_negative CHECK (max_retries >= 0)
);
