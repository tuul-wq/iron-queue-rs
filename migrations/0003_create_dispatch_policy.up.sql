CREATE TABLE dispatch_policy (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    policy JSONB NOT NULL CHECK (
        jsonb_typeof(policy) = 'object'
        AND policy ? 'type'
        AND jsonb_typeof(policy -> 'type') = 'string'
    ),

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO dispatch_policy (policy) VALUES (
    '{
      "type": "quota",
      "high": 6,
      "normal": 3,
      "low": 1
    }'::jsonb
);
