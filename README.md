# iron-queue-rs

A PostgreSQL-backed background job queue written in Rust.

The project is a standalone backend service: an HTTP API accepts jobs, stores them in PostgreSQL, and internal workers execute those jobs asynchronously outside the request lifecycle.

## Core Idea

```text
Client -> API -> PostgreSQL -> Worker -> Job status update
```

The API does not perform long-running work directly. It creates jobs and returns immediately. Workers poll PostgreSQL, safely claim eligible jobs, simulate execution, and update job status.

## Goals

The MVP is complete when the system can demonstrate this end-to-end flow:

1. Submit a job with `curl`.
2. A worker claims the job without double-processing, even with concurrent workers.
3. The worker executes simulated async work.
4. The job status is observable through the API.
5. Retries, priority ordering, queue concurrency limits, cancellation, delayed execution, and dead-lettering are visible through API responses.

## Tech Stack

- Rust
- Tokio for async runtime
- Axum for the HTTP API
- PostgreSQL for persistent queue storage
- SQLx for database access
- Serde for request, response, and JSON payload serialization
- thiserror for structured error handling
- tracing for structured logging
- Docker Compose for the target local development stack

## API

The service exposes an open REST API. Authentication and authorization are out of scope for the MVP.

| Method | Endpoint | Description |
| --- | --- | --- |
| `POST` | `/jobs` | Enqueue a new job with a queue name, JSON payload, optional priority, and optional `run_at` timestamp |
| `GET` | `/jobs` | List all jobs with their current status |
| `GET` | `/jobs/{id}` | Get status and details for one job |
| `POST` | `/jobs/{id}/cancel` | Request best-effort cancellation for a queued or running job |
| `GET` | `/workers` | List active workers and their current state |

## Job Submission

Jobs are scoped to a named queue. Each queue can define its own expected payload shape, and payloads are stored as JSONB in PostgreSQL.

```bash
curl -X POST http://localhost:3000/jobs \
  -H "Content-Type: application/json" \
  -d '{"queue":"reports","payload":{"report_id":42}}'
```

Optional fields:

- `priority`: integer priority, where higher values are processed first within a queue.
- `run_at`: future timestamp for scheduled or delayed execution.

Example with priority and delayed execution:

```bash
curl -X POST http://localhost:3000/jobs \
  -H "Content-Type: application/json" \
  -d '{"queue":"reports","payload":{"report_id":99},"priority":10,"run_at":"2025-06-01T10:00:00Z"}'
```

Check a job later:

```bash
curl http://localhost:3000/jobs/{id}
```

## Job Lifecycle

```text
QUEUED -> RUNNING -> DONE
                 -> FAILED -> retry with backoff -> QUEUED
                           -> retries exhausted -> DEAD_LETTERED
                 -> CANCELLED

SCHEDULED -> QUEUED when run_at has passed
```

Status meanings:

- `SCHEDULED`: the job has a future `run_at` and is not eligible for workers yet.
- `QUEUED`: the job is ready to be claimed.
- `RUNNING`: a worker has claimed the job and is executing it.
- `DONE`: the job completed successfully.
- `FAILED`: the job failed and may be retried.
- `DEAD_LETTERED`: the job exhausted retries and remains queryable.
- `CANCELLED`: cancellation was requested and applied on a best-effort basis.

## Worker Behavior

Workers are internal long-running processes. Multiple workers can run at the same time and compete for jobs.

Workers are expected to:

- Poll PostgreSQL for eligible jobs.
- Use database locking, such as `SELECT ... FOR UPDATE SKIP LOCKED`, to avoid double-processing.
- Ignore jobs scheduled for the future.
- Respect per-queue concurrency limits before claiming more work.
- Pick the highest-priority eligible job first within a queue.
- Execute simulated async work using randomized sleep and random failure.
- Retry failed jobs with backoff until max attempts are exhausted.
- Move exhausted jobs to `DEAD_LETTERED` without deleting or automatically re-queuing them.

## Queue Features

Named queues:
Jobs are submitted to queues such as `email`, `reports`, or `notifications`. Each queue can have its own payload shape.

Priority:
Workers prefer higher-priority eligible jobs within the same queue.

Scheduled jobs:
Jobs with a future `run_at` start in `SCHEDULED` and become eligible only after that timestamp has passed.

Retries:
Failed jobs are retried up to a configurable maximum number of attempts with a backoff delay.

Dead-letter queue:
Jobs that exhaust retries become `DEAD_LETTERED`. They remain visible through `GET /jobs` and `GET /jobs/{id}`.

Per-queue concurrency limits:
Each named queue can cap how many jobs may run in parallel.

Cancellation:
`POST /jobs/{id}/cancel` makes a best-effort attempt to cancel queued or in-progress jobs.

## Local Development

Create an environment file:

```bash
cp .env.example .env
```

Start PostgreSQL using a database that matches `DATABASE_URL` in `.env`:

```text
postgres://iron_q_up:iron_q_down@localhost:5432/iron_queue
```

Start the Dockerized PostgreSQL service:

```bash
docker compose up -d postgres
```

Run migrations:

```bash
sqlx migrate run --source src/migrations
```

Run the API:

```bash
cargo run --bin api
```

Run a worker when the worker binary is available:

```bash
cargo run --bin worker
```

## Example Workflows

Submit a basic job:

```bash
curl -X POST http://localhost:3000/jobs \
  -H "Content-Type: application/json" \
  -d '{"queue":"reports","payload":{"report_id":42}}'
```

Submit a scheduled job:

```bash
curl -X POST http://localhost:3000/jobs \
  -H "Content-Type: application/json" \
  -d '{"queue":"reports","payload":{"report_id":99},"run_at":"2025-06-01T10:00:00Z"}'
```

Cancel a job:

```bash
curl -X POST http://localhost:3000/jobs/{id}/cancel
```

List jobs:

```bash
curl http://localhost:3000/jobs
```

Inspect workers:

```bash
curl http://localhost:3000/workers
```

## MVP Scope

In scope:

- REST API for job creation, listing, inspection, cancellation, and worker status.
- Named queues with JSONB payloads stored in PostgreSQL.
- Concurrent workers with distributed job locking.
- Simulated job execution.
- Retry with backoff.
- Priority-based job selection.
- Per-queue concurrency limits.
- Dead-lettered jobs that remain queryable.
- Scheduled and delayed jobs through `run_at`.
- Local development with PostgreSQL, API, and workers.

Out of scope for the MVP:

- Authentication and authorization.
- Real integrations such as email, file processing, or report generation.
- WebSocket or Server-Sent Events updates.
- CLI tooling for queue inspection.
- Metrics and observability dashboards.
- Manual re-queueing of dead-lettered jobs.

## Stretch Goals

- Real-time job status updates with WebSocket or Server-Sent Events.
- Manual re-queueing of dead-lettered jobs through the API.
- CLI tooling for queue inspection and management.
- Job history and audit logs.
- Rate limiting per queue.
