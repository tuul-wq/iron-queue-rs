# iron-queue-rs

A PostgreSQL-backed background job queue written in Rust.

The goal of this project is to learn practical Rust backend development through a small but real system: an API accepts jobs, stores them in PostgreSQL, and workers execute them asynchronously.

---

## Core Idea

```text
Client -> API -> PostgreSQL -> Worker -> Job status update
```

The API does not execute long-running work directly.
It creates a job and returns immediately.

Workers pick queued jobs, execute them, and update their status.

---

## Tech Stack

* Rust
* Axum
* Tokio
* PostgreSQL
* SQLx
* Serde
* thiserror
* tracing
* Docker Compose

---

## Job Lifecycle

```text
queued -> running -> succeeded
queued -> running -> failed -> queued
queued -> running -> failed -> dead
queued -> cancelled
```

---

## Planned API

```http
POST /jobs
GET /jobs/{id}
GET /jobs
POST /jobs/{id}/cancel
GET /workers
```

---

## Example Job

```json
{
  "kind": "send_email",
  "payload": {
    "to": "user@example.com",
    "subject": "Welcome"
  },
  "priority": 5,
  "max_retries": 3
}
```

---

## MVP Scope

* create jobs through REST API
* store jobs in PostgreSQL
* list and inspect jobs
* run worker process
* claim queued jobs safely
* mark jobs as succeeded or failed
* retry failed jobs
* move exhausted jobs to `dead`
* track worker heartbeat
* recover stuck jobs

---

## Local Development

Start PostgreSQL:

```bash
docker compose up -d
```

Run migrations:

```bash
sqlx migrate run
```

Run API:

```bash
cargo run --bin api
```

Run worker:

```bash
cargo run --bin worker
```
