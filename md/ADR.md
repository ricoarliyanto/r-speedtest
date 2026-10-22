# Architecture Decision Records (ADR)
## Network Swiss Army & Speedtest Diagnostic Platform

---

## ADR Index

| ADR ID | Title | Status | Date |
| :--- | :--- | :--- | :--- |
| **ADR-001** | Adopting QiLuo as Core Application Foundation | **Accepted** | 2026-08-22 |
| **ADR-002** | Using DragonflyDB as Redis Drop-in Replacement | **Accepted** | 2026-08-22 |
| **ADR-003** | Whitelist-Only Typed Process Execution (Zero `sh -c`) | **Accepted** | 2026-08-22 |
| **ADR-004** | Decoupled Background Worker Architecture via Queue | **Accepted** | 2026-08-22 |
| **ADR-005** | Real-Time Telemetry via WebSocket over SSE | **Accepted** | 2026-08-22 |
| **ADR-006** | Native Rust Implementation for TR-143 / UDP Echo | **Accepted** | 2026-08-22 |
| **ADR-007** | Bounded Ephemeral Storage & Retention Policy for PCAP Files | **Accepted** | 2026-08-22 |

---

## ADR-001: Adopting QiLuo as Core Application Foundation

### Context
Building a multi-tenant, enterprise-grade network diagnostic platform requires robust infrastructure: User Authentication (JWT), granular Role-Based Access Control (RBAC), Operation/Audit Logging, Scheduled/Cron jobs, Database ORM mappings, and a polished administrative UI. Developing these from scratch would take months and divert focus from network engineering capabilities.

### Decision
We adopt **QiLuo** (`qiluo_admin`, `qiluo_vue`, `qiluo_tauri`) as the baseline application framework. We build modular extensions (`net_tool`, `net_job`, `net_scenario`, `net_node`) on top of QiLuo’s Axum routing, Sea-ORM database layer, and Vue 3 frontend components.

### Consequences
- **Positive**: Instant access to enterprise RBAC, audit logging, menu permissioning, and desktop packaging via Tauri.
- **Positive**: Standardized Rust + Axum + Sea-ORM ecosystem with minimal boilerplate.
- **Negative**: Must adhere to QiLuo's module patterns and database schema naming conventions.

---

## ADR-002: Using DragonflyDB as Redis Drop-in Replacement

### Context
Speedtesting and high-frequency network metrics require an in-memory datastore capable of handling thousands of telemetry events per second, managing asynchronous job queues, and distributing Pub/Sub messages without blocking the main event loop.

### Decision
We use **DragonflyDB** as our in-memory engine, fully compatible with the Redis 6/7 protocol.

### Consequences
- **Positive**: Up to $25\times$ higher throughput and significantly lower tail latency compared to standard Redis under heavy multi-threaded concurrent load.
- **Positive**: 100% API compatibility with `redis-rs` and `bb8-redis` crates used by QiLuo.
- **Positive**: Ultra-low memory overhead with native lock-free multi-threaded architecture.
- **Negative**: Requires modern Linux kernel features (e.g. `io_uring`) for peak performance.

---

## ADR-003: Whitelist-Only Typed Process Execution (Zero `sh -c`)

### Context
Network diagnostics require running system-level binaries (`ping`, `traceroute`, `mtr`, `iperf3`, `tcpdump`). Traditional web wrappers often use shell interpolation (`sh -c format!("ping {}", user_input)`), which is the #1 vector for critical Command Injection vulnerabilities (CWE-78).

### Decision
Arbitrary shell execution is strictly prohibited. All tool executions must:
1. Implement the `NetworkTool` trait.
2. Bind to explicit binary paths (e.g., `/usr/bin/ping`).
3. Build arguments via structured vectors (`Vec<String>`) using strictly validated and sanitized parameters.
4. Execute via Tokio's asynchronous `Command` API without a shell wrapper.

### Consequences
- **Positive**: Eliminates command injection risks entirely.
- **Positive**: Allows precise signal passing (`SIGINT`, `SIGTERM`) for instant job cancellation.
- **Negative**: Cannot use shell pipes (`|`) or redirections (`>`) directly; all output processing must be handled in Rust parsers.

---

## ADR-004: Decoupled Background Worker Architecture via Queue

### Context
Performance tests (e.g. 60-second iPerf3 or continuous TR-143 download) are long-running. Running these synchronously inside HTTP request handlers causes connection timeouts, blocks HTTP worker threads, and prevents horizontal scaling.

### Decision
All tool runs are dispatched as **Jobs**.
1. The API server writes the Job to PostgreSQL and pushes a work ticket into a DragonflyDB FIFO queue (`queue:net_tool`).
2. Dedicated async worker instances (`NetToolWorker`) pull jobs using `BRPOP`.
3. The worker updates job status and streams progress back via Pub/Sub.

```
[ Axum API Server ] ---> ( PostgreSQL ) + ( DragonflyDB Queue )
                                                  |
                                             [ Worker ] ---> [ Network Tool Execution ]
```

### Consequences
- **Positive**: HTTP API remains lightning-fast ($< 20\text{ms}$ response time returning `202 Accepted` with `job_id`).
- **Positive**: Workers can be scaled independently or hosted on remote edge probe servers.
- **Positive**: Workload surges do not degrade web dashboard responsiveness.

---

## ADR-005: Real-Time Telemetry via WebSocket over SSE

### Context
Speedtests and live diagnostics (MTR, Ping, iPerf3) require real-time graph updates at $2 \sim 5\text{ Hz}$. We considered Server-Sent Events (SSE) and WebSocket.

### Decision
We use **WebSocket** (`axum::extract::ws`) for live telemetry delivery.

### Consequences
- **Positive**: Full-duplex channel allows dynamic client subscriptions (e.g. client can subscribe/unsubscribe to multiple running jobs on the fly without reconnecting).
- **Positive**: Native bidirectional support across web browsers and Tauri desktop applications.
- **Positive**: Low frame overhead compared to repeated HTTP/SSE header parsing.
- **Negative**: Requires explicit heartbeat / ping-pong handling to detect dead client sockets.

---

## ADR-006: Native Rust Implementation for TR-143 & UDP Echo

### Context
TR-143 (Broadband Forum diagnostic standard for ONT/ACS) requires HTTP Download/Upload throughput metrics and RFC 862 UDP/TCP Echo. Relying on external third-party daemons introduces binary dependency mismatch, deployment friction, and poor telemetry integration.

### Decision
We implement TR-143 diagnostics **natively in Rust** using `tokio`, `hyper`, and zero-copy chunk generators for download/upload and microsecond-timestamped UDP sockets for echo tests.

### Consequences
- **Positive**: Extreme memory and CPU efficiency capable of saturating 10Gbps+ interfaces.
- **Positive**: No external compilation or packaging dependencies.
- **Positive**: Direct access to microsecond timing measurements (`BOMTime`, `EOMTime`, `ROMTime`).

---

## ADR-007: Bounded Ephemeral Storage & Retention Policy for PCAP Files

### Context
Packet captures (`tcpdump`) can rapidly consume tens of gigabytes of disk space, posing a Denial of Service risk to the host filesystem.

### Decision
1. PCAP captures are strictly capped at maximum 250MB or 300 seconds per execution.
2. PCAP artifacts are saved to a dedicated directory `/var/log/r-speedtest/pcaps/` with non-guessable UUID filenames.
3. An automated background cleanup worker deletes capture files older than 48 hours.

### Consequences
- **Positive**: Prevents disk exhaustion on production servers.
- **Positive**: Complies with data minimization privacy standards.
