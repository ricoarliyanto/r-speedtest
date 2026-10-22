# Implementation Plan: Milestone 1 — Core Foundation & Deployment Setup

This plan details the technical execution of **Milestone 1** from [MileStones.md](file:///home/win/code/r-speedtest/md/MileStones.md), establishing the deployment containers, database schema migrations, and model entities for the Speedtest & Network Swiss Army platform.

---

## User Review Required

> [!IMPORTANT]
> **What will be created/modified in Milestone 1:**
> 1. Root deployment files: [`docker-compose.yml`](file:///home/win/code/r-speedtest/docker-compose.yml), [`Dockerfile`](file:///home/win/code/r-speedtest/Dockerfile), and [`Caddyfile`](file:///home/win/code/r-speedtest/Caddyfile).
> 2. Sea-ORM migration file in [`qiluo_admin/migration/src/`](file:///home/win/code/r-speedtest/qiluo_admin/migration/src/) for tables: `net_tools`, `net_nodes`, `net_jobs`, `net_job_results`, `net_scenarios`, `net_scenario_steps`.
> 3. Sea-ORM model entities in [`qiluo_admin/src/model/net/`](file:///home/win/code/r-speedtest/qiluo_admin/src/model/net/).
> 4. DragonflyDB queue registration (`queue:net_tool`, `queue:scenario`) in [`processor_manager.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/processor_manager.rs).

---

## Proposed Changes

### 1. Root Deployment & Containerization

#### [NEW] [docker-compose.yml](file:///home/win/code/r-speedtest/docker-compose.yml)
- Services: `postgres` (16-alpine), `dragonfly` (DragonflyDB high-speed cache/queue), `app` (QiLuo Axum backend + web static assets), `worker` (isolated network diagnostic worker with `NET_RAW` / `NET_ADMIN` capabilities), `caddy` (reverse proxy + WebSocket WSS proxy).

#### [NEW] [Dockerfile](file:///home/win/code/r-speedtest/Dockerfile)
- Stage 1: Build Vue 3 Frontend with `pnpm build:pro`.
- Stage 2: Build Rust backend binary with Cargo.
- Stage 3: Runtime Debian slim container with installed network utilities (`iputils-ping`, `traceroute`, `mtr-tiny`, `iperf3`, `tcpdump`, `dnsutils`, `curl`).

#### [NEW] [Caddyfile](file:///home/win/code/r-speedtest/Caddyfile)
- Reverse proxy configuration with automatic TLS, SPA routing, and `/api/net/ws` WebSocket upgrade.

---

### 2. Database Schema & Sea-ORM Migration (`qiluo_admin/migration`)

#### [NEW] [m20260822_000001_create_net_tables.rs](file:///home/win/code/r-speedtest/qiluo_admin/migration/src/m20260822_000001_create_net_tables.rs)
- DDL for:
  - `net_tools`: `id`, `name`, `category`, `binary_path`, `input_schema_json`, `output_schema_json`, `default_timeout_sec`, `is_active`, `required_permission`.
  - `net_nodes`: `id`, `name`, `ip_address`, `region`, `status`, `last_heartbeat`, `capabilities`, `auth_token_hash`.
  - `net_jobs`: `id`, `tool_id`, `node_id`, `scenario_id`, `target_host`, `parameters`, `status`, `requested_by`, `created_at`, `started_at`, `finished_at`, `duration_sec`.
  - `net_job_results`: `id`, `job_id`, `structured_metrics`, `raw_output`, `artifact_path`, `exit_code`, `error_message`.
  - `net_scenarios`: `id`, `name`, `description`, `stop_on_failure`, `execution_flow`, `created_by`.
  - `net_scenario_steps`: `id`, `scenario_id`, `step_order`, `tool_id`, `step_parameters`, `condition_rules`.

#### [MODIFY] [lib.rs](file:///home/win/code/r-speedtest/qiluo_admin/migration/src/lib.rs)
- Register `m20260822_000001_create_net_tables` in the migration migrator vector.

---

### 3. Sea-ORM Backend Model Entities (`qiluo_admin/src/model/net/`)

#### [NEW] [net_tool.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/model/net/net_tool.rs)
- Sea-ORM Entity for `net_tools`.

#### [NEW] [net_job.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/model/net/net_job.rs)
- Sea-ORM Entity for `net_jobs`.

#### [NEW] [net_job_result.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/model/net/net_job_result.rs)
- Sea-ORM Entity for `net_job_results`.

#### [NEW] [net_node.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/model/net/net_node.rs)
- Sea-ORM Entity for `net_nodes`.

#### [NEW] [net_scenario.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/model/net/net_scenario.rs)
- Sea-ORM Entity for `net_scenarios` and `net_scenario_steps`.

#### [NEW] [mod.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/model/net/mod.rs)
- Export model modules.

#### [MODIFY] [model.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/model.rs)
- Add `pub mod net;` to QiLuo models.

---

### 4. DragonflyDB Queue Registration (`qiluo_admin/src/worker/`)

#### [MODIFY] [processor_manager.rs](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/processor_manager.rs)
- Append `"net_tool"` and `"net_scenario"` to `DEFAULT_QUEUES`.

---

## Verification Plan

### Automated Tests & Checks
- Run Sea-ORM migration dry-run / check:
  ```bash
  cargo check --manifest-path qiluo_admin/Cargo.toml
  ```
- Validate Docker Compose syntax:
  ```bash
  docker compose config
  ```

### Manual Verification
- Verify that models compile cleanly with Sea-ORM and all relations are properly linked.
