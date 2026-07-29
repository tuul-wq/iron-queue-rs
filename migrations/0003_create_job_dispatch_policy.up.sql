CREATE TYPE dispatch_strategy AS ENUM (
    'quota',
    'aging'
);

CREATE TABLE job_dispatch_policy (
    id SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),

    policy JSONB NOT NULL CHECK (
        jsonb_typeof(policy) = 'object'
        AND policy ? 'strategy'
        AND jsonb_typeof(policy -> 'strategy') = 'string'
    ),

    revision INT NOT NULL DEFAULT 1 CHECK (revision > 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
