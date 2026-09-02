# iron-queue-rs

A PostgreSQL-backed background job queue written in Rust.

The API stores jobs and returns immediately. Workers claim jobs from PostgreSQL, execute simulated handlers, and persist the resulting status.

```text
Client -> API -> PostgreSQL -> Worker
```

## Current Features

- Persistent jobs with `queued`, `running`, `completed`, and `failed` states.
- Atomic job claiming with PostgreSQL row locking and `SKIP LOCKED`.
- Priority-aware dispatch using quota or aging policies.
- Retryable failures with exponential backoff.
- Simulated `send_email` and `generate_report` job handlers.
- Runtime dispatch-policy updates delivered to workers through PostgreSQL notifications.

## API

| Method | Endpoint | Description |
| --- | --- | --- |
| `GET` | `/health` | Health check |
| `POST` | `/jobs` | Create a job |
| `GET` | `/jobs` | List jobs |
| `GET` | `/jobs/{id}` | Get a job |
| `GET` | `/dispatch_policy` | Get the active dispatch policy |
| `POST` | `/dispatch_policy` | Add a dispatch policy revision |
| `GET` | `/dispatch_policy/history` | List policy revisions |

Create an email job:

```bash
curl -X POST http://127.0.0.1:3000/jobs \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "welcome-email",
    "priority": "high",
    "max_retries": 3,
    "payload": {
      "SendEmail": {
        "to": "user@example.com",
        "subject": "Welcome",
        "template_id": "welcome",
        "variables": {"name": "Ada"}
      }
    }
  }'
```

Supported payload variants are `SendEmail` and `GenerateReport`. Priorities are `low`, `normal`, and `high`.

## Dispatch Policies

The active policy controls how workers choose their next job. New revisions are stored in PostgreSQL and loaded by running workers.

```bash
curl -X POST http://127.0.0.1:3000/dispatch_policy \
  -H 'Content-Type: application/json' \
  -d '{"policy":{"type":"quota","high":6,"normal":3,"low":1}}'
```

`quota` cycles through preferred priorities according to the configured weights. `aging` raises a job's effective priority as it waits.

## How Dispatch Works

### Quota

For a `6/3/1` policy, a worker prefers jobs in this repeating order:

```text
high -> high -> high -> high -> high -> high -> normal -> normal -> normal -> low -> repeat
```

If the preferred priority has no eligible jobs, the worker claims the next available job instead.

### Aging

Aging prevents lower-priority jobs from waiting indefinitely. The worker ranks eligible jobs by:

```text
effective priority = base priority + floor(wait time / aging_step_seconds)

high:   2 + age
normal: 1 + age
low:    0 + age
```

With `aging_step_seconds: 5`, a normal-priority job waiting 10 seconds has a score of `3`, so it outranks a newly created high-priority job with a score of `2`.

```bash
curl -X POST http://127.0.0.1:3000/dispatch_policy \
  -H 'Content-Type: application/json' \
  -d '{"policy":{"type":"aging","aging_step_seconds":5}}'
```

## Run Locally

Prerequisites: Rust, Docker Compose, PostgreSQL, and the SQLx CLI.

```bash
cp .env.example .env
docker compose up -d postgres
sqlx migrate run --source migrations
```

Run the API and worker in separate terminals:

```bash
cargo run --bin api
cargo run --bin worker
```

Optionally seed the queue with sample jobs:

```bash
cargo run --bin seed
```
