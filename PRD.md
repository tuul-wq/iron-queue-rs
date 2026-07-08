# Product Requirements Document — PostgreSQL-Backed Background Job Queue

## 1. Problem Statement

Developers building backend systems often need to defer long-running or asynchronous work outside of the HTTP request lifecycle. This project implements a standalone background job queue service with an HTTP API, backed by PostgreSQL, where workers compete to execute jobs asynchronously.

---

## 2. Goals & Success Criteria

**The project is "done" when:**
- A working end-to-end system can be demonstrated: a job is submitted via `curl`, picked up by a worker, executed, and its status is observable via the API.
- The system correctly handles multiple concurrent workers competing for jobs without double-processing.
- Retry logic, priority ordering, and per-queue concurrency limits are functional and observable through simulated job failures.
- Dead-lettered jobs are visible via the API after exhausting all retries.
- Scheduled jobs are not picked up by workers before their `run_at` time.

## 3. Core Features (MVP)

### 3.1 Job Submission & Management API
A REST HTTP API with the following endpoints:

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/jobs` | Enqueue a new job with a queue name, JSONB payload, optional priority, and optional `run_at` timestamp |
| `GET` | `/jobs` | List all jobs with their current status |
| `GET` | `/jobs/{id}` | Get status and details of a specific job |
| `POST` | `/jobs/{id}/cancel` | Request cancellation of a job (best-effort — targets queued and in-progress jobs) |
| `GET` | `/workers` | List active workers and their current state (idle, busy, job count) |

- No authentication required — the API is fully open.
- Job payloads are stored as JSONB in PostgreSQL, with each named queue expecting its own payload shape.

### 3.2 Named Queues with Typed Payloads
- The system supports multiple named queues (e.g., `email`, `reports`, `notifications`).
- Each queue has its own payload structure stored as JSONB.
- Jobs are scoped to a queue at submission time.

### 3.3 Worker Loop
- Workers are long-running internal processes that poll PostgreSQL for queued jobs.
- Multiple worker processes can run simultaneously and compete for jobs.
- Job locking (e.g., `SELECT ... FOR UPDATE SKIP LOCKED`) prevents double-processing.
- Workers skip jobs whose `run_at` timestamp is in the future.
- Workers execute simulated work (e.g., a randomized sleep with a chance of random failure) to exercise real async behavior without requiring real integrations.

### 3.4 Job Status Lifecycle
Jobs move through the following states:

```
QUEUED → RUNNING → DONE
                 → FAILED → (retry with backoff) → QUEUED
                          → (retries exhausted)  → DEAD_LETTERED
                 → CANCELLED
SCHEDULED → QUEUED (when run_at time is reached)
```

### 3.5 Retry with Backoff
- Failed jobs are automatically retried up to a configurable maximum number of attempts.
- Each retry is delayed using a backoff strategy (e.g., exponential or fixed delay).
- Once all retries are exhausted, the job transitions to `DEAD_LETTERED`.

### 3.6 Priority Queues
- Jobs can be submitted with an optional priority value (e.g., integer — higher value = higher priority).
- Workers always pick the highest-priority eligible job first within a queue.

### 3.7 Per-Queue Concurrency Limits
- Each named queue has a configurable cap on how many jobs can run in parallel at any given time.
- Workers respect this limit before picking up a new job from a queue.

### 3.8 Dead-Letter Queue (DLQ)
- Jobs that exhaust all retry attempts transition to `DEAD_LETTERED` state.
- Dead-lettered jobs remain in the database and are queryable via `GET /jobs` and `GET /jobs/{id}`.
- They are never automatically deleted or re-queued.

### 3.9 Scheduled / Delayed Jobs
- `POST /jobs` accepts an optional `run_at` timestamp.
- If `run_at` is provided and in the future, the job is created in a `SCHEDULED` state.
- Workers will not pick up the job until the current time is past `run_at`, at which point it becomes eligible like any other queued job.

---

## 4. User Stories / Use Cases

### 4.1 Fire and Forget — Basic Job Submission
A developer submits a job via `curl`:
```bash
curl -X POST http://localhost:3000/jobs \
  -H "Content-Type: application/json" \
  -d '{"queue": "reports", "payload": {"report_id": 42}}'
```
The API returns immediately with a job ID. The developer optionally checks status later:
```bash
curl http://localhost:3000/jobs/{id}
```

### 4.2 Observing Retry and Dead-Letter Behavior
A job is submitted and picked up by a worker. The worker simulates a random failure. The developer polls `GET /jobs/{id}` and observes the job transition from `FAILED` back to `QUEUED`, then `RUNNING` across multiple attempts. After exhausting all retries, the job moves to `DEAD_LETTERED` and remains visible via the API.

### 4.3 Scheduled Job Execution
A developer submits a job with a future `run_at` timestamp:
```bash
curl -X POST http://localhost:3000/jobs \
  -H "Content-Type: application/json" \
  -d '{"queue": "reports", "payload": {"report_id": 99}, "run_at": "2025-06-01T10:00:00Z"}'
```
The job is created in `SCHEDULED` state and ignored by workers until the specified time has passed.

### 4.4 Cancellation
A developer submits a long-running job, then cancels it before (or during) execution:
```bash
curl -X POST http://localhost:3000/jobs/{id}/cancel
```
The system makes a best-effort attempt to cancel the job regardless of whether it is queued or already in progress.

### 4.5 Monitoring Workers
A developer checks which workers are active and what they are processing:
```bash
curl http://localhost:3000/workers
```
The response shows each worker's ID, current state, and how many jobs it is handling.

---

## 5. Scope & Constraints

### In Scope (MVP)
- REST HTTP API with the 5 endpoints listed above
- Named queues with JSONB payloads stored in PostgreSQL
- Multiple concurrent workers with distributed job locking
- Simulated job execution (randomized sleep + random failure rate)
- Retry with backoff on job failure
- Priority queues (higher-priority jobs are picked up first)
- Per-queue concurrency limits (cap on how many jobs run in parallel per queue)
- Dead-letter queue — jobs that exhaust retries move to `DEAD_LETTERED` state and remain queryable
- Scheduled / delayed job execution via optional `run_at` timestamp on `POST /jobs`
- Worker status reporting via `GET /workers`
- Local development environment via Docker Compose (Postgres + API + workers)

### Out of Scope (MVP)
- Authentication or authorization
- Real job integrations (email sending, file processing, etc.)
- WebSocket or Server-Sent Events for live status updates
- A CLI tool for queue inspection
- Metrics and observability (Prometheus, Grafana, etc.)
- Manual re-queuing of dead-lettered jobs

### External Dependencies
- PostgreSQL (via Docker Compose)
- [axum](https://github.com/tokio-rs/axum) — HTTP framework
- [tokio](https://github.com/tokio-rs/tokio) — async runtime
- [sqlx](https://github.com/launchbadge/sqlx) — async PostgreSQL driver
- [serde](https://github.com/serde-rs/serde) — serialization
- [thiserror](https://github.com/dtolnay/thiserror) — error handling
- [tracing](https://github.com/tokio-rs/tracing) — structured logging

### Constraints
- Workers are internal processes (not external plugins or user-defined code)
- No SLA on job execution time
- Cancellation of in-progress jobs is best-effort only
- Dead-lettered jobs are never automatically re-queued or deleted

---

## 6. Stretch Goals (Post-MVP)
- WebSocket or Server-Sent Events for real-time job status updates
- Manual re-queuing of dead-lettered jobs via the API
- A CLI tool for queue inspection and management
- Job history / audit log
- Rate limiting per queue
