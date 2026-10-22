# Detailed Implementation Milestones (MileStones.md)
## Network Swiss Army & Speedtest Diagnostic Platform

---

## Document Purpose & Overview

This document provides the definitive, task-by-task engineering roadmap derived from [`PHASES.md`](file:///home/win/code/r-speedtest/md/PHASES.md) and [`TDD.md`](file:///home/win/code/r-speedtest/md/TDD.md). Every milestone specifies exact files to create or modify, database schemas, API routes, worker mechanisms, security constraints, and explicit **Definition of Done (DoD)** criteria.

```
+---------------------------------------------------------------------------------------------------+
|                                  8-MILESTONE ROADMAP OVERVIEW                                      |
+-------------------+--------------------+--------------------+--------------------+----------------+
|  Milestone 1      |  Milestone 2       |  Milestone 3       |  Milestone 4       |  Milestones    |
|  Core Foundation  |  Tool Registry,    |  iPerf3 & UDP Echo |  Native TR-143     |  5 to 8        |
|  & QiLuo Extension|  WS Realtime Engine|  Speedtest Engine  |  ISP Server/Client |  Deep Diag,    |
|  (DB/Queue/Auth)  |  (Ping/Trace/DNS)  |  (10G Throughput)  |  (Broadband Forum) |  DAG, Nodes,NOC|
+-------------------+--------------------+--------------------+--------------------+----------------+
```

---

# Milestone 1: Core Foundation & QiLuo Custom Extension Setup

### 1.1 Objective
Establish the foundational infrastructure on top of the QiLuo admin base (`qiluo_admin`, `qiluo_vue`, `qiluo_tauri`), configure the PostgreSQL database schema and migrations via Sea-ORM, integrate DragonflyDB task queues, configure RBAC permissions, and scaffold frontend menu routes.

---

### 1.2 Step-by-Step Implementation Tasks

#### Task 1.1: Sea-ORM Migration & Entity Creation
- **Action**: Create migration file `qiluo_admin/migration/src/m20260822_000001_create_net_tables.rs`.
- **Tables to Create**:
  1. `net_tools`: Tool definitions, schemas, binary paths, default timeouts.
  2. `net_nodes`: Edge node definitions, IP addresses, regions, status, capabilities.
  3. `net_jobs`: Execution records, requested parameters, status, timestamps, duration.
  4. `net_job_results`: Structured metrics JSON, raw output, exit codes, artifact paths.
  5. `net_scenarios` & `net_scenario_steps`: Multi-step scenario definitions.
- **Model Entities**: Create `qiluo_admin/src/model/net/` (`net_tool.rs`, `net_node.rs`, `net_job.rs`, `net_job_result.rs`, `net_scenario.rs`).

#### Task 1.2: DragonflyDB Task Queue Configuration
- **Action**: Modify `qiluo_admin/src/worker/processor_manager.rs` and `development.yaml`.
- **Details**:
  - Register queues: `"net_tool"`, `"net_scenario"`, `"net_node_hb"`.
  - Configure connection pool to DragonflyDB on port `6379`.
  - Verify `BRPOP` polling behavior with 2-second timeout.

#### Task 1.3: RBAC Granular Permissions Seeding
- **Action**: Seed the following permissions into QiLuo’s `sys_menu` / `sys_role_menu` tables:
  - `net:tool:list`, `net:tool:execute`
  - `net:job:query`, `net:job:cancel`
  - `net:scenario:list`, `net:scenario:manage`, `net:scenario:run`
  - `net:node:list`, `net:node:manage`
  - `net:tcpdump:execute`, `net:tcpdump:download`

#### Task 1.4: Frontend Navigation & Route Scaffolding
- **Action**: Add route definitions and menu structure in `qiluo_vue/src/router/routes/modules/network.ts`:
  - `/network/tools`: Network Diagnostic Console.
  - `/network/speedtest`: iPerf3 & TR-143 Speedtest Gauges.
  - `/network/scenarios`: Diagnostic Scenarios & Automation.
  - `/network/jobs`: Execution History & Metrics Log.
  - `/network/nodes`: Multi-Region Edge Node Management.

---

### 1.3 File Changes Matrix (Milestone 1)

| File Path | Action | Description |
| :--- | :--- | :--- |
| `qiluo_admin/migration/src/m20260822_000001_create_net_tables.rs` | **NEW** | Sea-ORM database migration DDL for network tables. |
| `qiluo_admin/migration/src/lib.rs` | **MODIFY** | Register new migration in migration vector. |
| `qiluo_admin/src/model/net/mod.rs` | **NEW** | Module export for network models. |
| `qiluo_admin/src/model/net/net_tool.rs` | **NEW** | Sea-ORM Entity for `net_tools`. |
| `qiluo_admin/src/model/net/net_job.rs` | **NEW** | Sea-ORM Entity for `net_jobs`. |
| `qiluo_admin/src/model/net/net_job_result.rs` | **NEW** | Sea-ORM Entity for `net_job_results`. |
| `qiluo_admin/src/model/net/net_node.rs` | **NEW** | Sea-ORM Entity for `net_nodes`. |
| `qiluo_admin/src/model/net/net_scenario.rs` | **NEW** | Sea-ORM Entity for `net_scenarios`. |
| `qiluo_admin/src/worker/processor_manager.rs` | **MODIFY** | Append `"net_tool"` and `"net_scenario"` to default worker queues. |
| `qiluo_vue/src/router/routes/modules/network.ts` | **NEW** | Vue Router definitions for network operational pages. |

---

### 1.4 Definition of Done (DoD) — Milestone 1
- [ ] `cargo run --manifest-path migration/Cargo.toml` executes cleanly on PostgreSQL and SQLite.
- [ ] Backend starts without error (`cargo run`) and logs DragonflyDB queue listener connection.
- [ ] RBAC permissions appear in QiLuo’s Role Permission Management UI.
- [ ] Vue 3 admin dashboard displays the "Network Operations" sidebar menu with all sub-routes.

---

# Milestone 2: Tool Registry, Job Engine & Real-Time Telemetry

### 2.1 Objective
Build the secure tool execution engine (`NetworkTool` trait), the asynchronous DragonflyDB queue worker (`NetToolWorker`), the WebSocket telemetry broadcaster, and implement foundational connectivity diagnostic tools (**Ping**, **Traceroute**, **DNS Lookup**, **Port Check**).

---

### 2.2 Step-by-Step Implementation Tasks

#### Task 2.1: Tool Trait Abstraction & Registry
- **Action**: Create `qiluo_admin/src/tools/mod.rs` and `registry.rs`.
- **Interface**:
  ```rust
  #[async_trait]
  pub trait NetworkTool: Send + Sync {
      fn id(&self) -> &'static str;
      fn binary(&self) -> &'static str;
      fn build_args(&self, ctx: &ToolContext) -> Result<Vec<String>>;
      fn parse_output(&self, stdout: &str, stderr: &str) -> Result<serde_json::Value>;
      fn max_timeout(&self) -> u64;
  }
  ```
- **Constraint**: Strict prohibition of arbitrary shell (`no sh -c`). All arguments must be structured `Vec<String>`.

#### Task 2.2: Implement Diagnostic Tools
1. **Ping Tool (`src/tools/ping.rs`)**:
   - Validates IPv4/IPv6/FQDN target.
   - Arguments: `-c <count>`, `-s <size>`, `-W <timeout>`, `-i <interval>`.
   - Parser: Extracts transmitted, received, packet loss %, min/avg/max/stddev RTT.
2. **Traceroute Tool (`src/tools/traceroute.rs`)**:
   - Arguments: `-m <max_hops>`, `-w <timeout>`, `-q <probes>`, `<target>`.
   - Parser: Extracts hop number, router IP, reverse DNS hostname, and per-probe RTT.
3. **DNS Lookup Tool (`src/tools/dns.rs`)**:
   - Custom resolver server option, record type selector (`A`, `AAAA`, `MX`, `TXT`, `PTR`).
   - Parser: Extract query time, TTL, answers section.
4. **Port Check Tool (`src/tools/portcheck.rs`)**:
   - TCP 3-way handshake timer, connection state (Open, Closed, Filtered).

#### Task 2.3: Asynchronous Job Worker (`NetToolWorker`)
- **Action**: Create `qiluo_admin/src/worker/net_tool_worker.rs`.
- **Workflow**:
  1. Pops job payload `{job_id, tool_id, target, params}` from `queue:net_tool`.
  2. Updates `net_jobs.status = 'running'` and `started_at = NOW()`.
  3. Executes binary via `tokio::process::Command` with piped stdout/stderr.
  4. Publishes live chunk events to DragonflyDB channel `channel:job:<job_id>`.
  5. On completion, parses output, writes `net_job_results`, updates status to `'completed'` or `'failed'`, and logs audit trail.

#### Task 2.4: WebSocket Hub & Streaming Handler
- **Action**: Create `qiluo_admin/src/api/net_ws_api.rs`.
- **Details**:
  - Endpoint: `GET /api/net/ws?token=<JWT>`.
  - Supports client subscription message: `{"action": "subscribe", "topic": "job:<job_id>"}`.
  - Bridges DragonflyDB Redis Pub/Sub events directly to subscribed WebSocket sockets.

#### Task 2.5: Vue 3 Diagnostic Console
- **Action**: Create `qiluo_vue/src/views/Network/DiagnosticConsole.vue`.
- **Features**: Target input bar, tool selector, parameter modal, live RTT sparkline chart, hop table for traceroute, and raw terminal log view.

---

### 2.3 API Specifications (Milestone 2)

| Method | Path | Request Body | Response JSON | Permission |
| :--- | :--- | :--- | :--- | :--- |
| `POST` | `/api/net/tools/:tool/run` | `{"target": "1.1.1.1", "params": {"count": 10, "size": 64}}` | `{"code": 200, "data": {"job_id": "01J8A..."}}` | `net:tool:execute` |
| `GET` | `/api/net/jobs/:id` | None | `{"code": 200, "data": {"job": {...}, "result": {...}}}` | `net:job:query` |
| `POST` | `/api/net/jobs/:id/cancel`| None | `{"code": 200, "message": "Job cancelled"}` | `net:job:cancel` |
| `WS` | `/api/net/ws` | Handshake with JWT | Bi-directional JSON stream | Authenticated |

---

### 2.4 Definition of Done (DoD) — Milestone 2
- [ ] Submitting Ping or Traceroute via API returns `202/200` with `job_id` in $< 30\text{ms}$.
- [ ] Worker executes the tool in the background; progress and stdout stream live to the Vue 3 console via WebSocket.
- [ ] Invalid targets (e.g., `8.8.8.8; rm -rf /`) are rejected by parameter validation before reaching the worker.
- [ ] Full structured output stored in `net_job_results` with exit code and execution duration.

---

# Milestone 3: High-Throughput Speedtest Engine (iPerf3 & UDP Echo)

### 3.1 Objective
Implement multi-gigabit throughput speedtesting capabilities using an **iPerf3** JSON stream wrapper and a high-precision **UDP Echo** latency/jitter engine (RFC 3550).

---

### 3.2 Step-by-Step Implementation Tasks

#### Task 3.1: iPerf3 Controller & JSON Parser
- **Action**: Create `qiluo_admin/src/tools/iperf3.rs`.
- **Features**:
  - Controlled Server Daemon mode on configurable ports (default 5201).
  - Client Mode arguments: `-c <target>`, `-p <port>`, `-t <duration>`, `-P <streams>`, `-J` (JSON mode).
  - Protocols: TCP (window size, BBR/Cubic) and UDP (`-u -b <bandwidth>`).
  - Reverse mode (`-R`) for server-to-client download testing.
  - JSON parser: Extracts intervals array (throughput per second, retransmits, CWND, jitter, packet loss).

#### Task 3.2: High-Precision UDP Echo Diagnostic Engine
- **Action**: Create `qiluo_admin/src/tools/udpecho.rs`.
- **Features**:
  - Microsecond timestamp packet generator.
  - Tracks sequence numbers, calculates: One-way delay, Round-trip latency, Out-of-order packets, Jitter via RFC 3550 formula:
    $$J(i) = J(i-1) + \frac{|D(i-1, i)| - J(i-1)}{16}$$

#### Task 3.3: Vue 3 Speedtest Dashboard & Gauges
- **Action**: Create `qiluo_vue/src/views/Network/Speedtest.vue`.
- **Components**:
  - ECharts tachometer speedometer gauge showing live Mbps/Gbps.
  - Real-time line chart comparing Download vs Upload throughput over test duration.
  - Metric summary cards: Peak Speed, Average Speed, Jitter (ms), Packet Loss (%).

---

### 3.3 Definition of Done (DoD) — Milestone 3
- [ ] iPerf3 TCP speedtest achieves line rate throughput (tested up to 1Gbps / 10Gbps locally or in staging).
- [ ] Second-by-second bandwidth metrics update the Vue 3 speedometer at $\ge 2\text{ Hz}$ frequency.
- [ ] UDP Echo test accurately measures sub-millisecond jitter and packet loss percentage.
- [ ] Reverse mode testing correctly differentiates download from upload streams.

---

# Milestone 4: Carrier-Grade ISP Diagnostics (Native TR-143 Engine)

### 4.1 Objective
Implement native, carrier-grade **TR-143 (Broadband Forum)** speedtest infrastructure in Rust for ISP, Telco, and ONT/CPE diagnostics without external binary dependencies.

---

### 4.2 Step-by-Step Implementation Tasks

#### Task 4.1: Native TR-143 HTTP Download Endpoint
- **Action**: Create `qiluo_admin/src/api/tr143_api.rs` and `qiluo_admin/src/tools/tr143/download.rs`.
- **Endpoint**: `GET /diagnostics/tr143/download?size=<bytes>`.
- **Implementation**:
  - Pre-allocated 64MB zero-copy circular buffer in memory.
  - Streams synthetic payload via Axum chunked response body.
  - Tracks client connection time, `BOMTime` (Beginning of Message), and `EOMTime` (End of Message).

#### Task 4.2: Native TR-143 HTTP Upload Endpoint
- **Action**: Create `qiluo_admin/src/tools/tr143/upload.rs`.
- **Endpoint**: `POST /diagnostics/tr143/upload`.
- **Implementation**:
  - Reads request stream directly into kernel black-hole buffer, avoiding disk write bottlenecks.
  - Calculates throughput:
    $$\text{Throughput} = \frac{\text{BytesReceived} \times 8}{\text{EOMTime} - \text{BOMTime}}$$

#### Task 4.3: TR-143 UDP/TCP Echo Server Daemons
- **Action**: Create `qiluo_admin/src/tools/tr143/echo.rs`.
- **Implementation**:
  - Tokio UDP socket listening on port `37001` (RFC 862 / TR-143 UDP Echo).
  - High-performance TCP loopback daemon on port `37002`.

#### Task 4.4: TR-143 Client Diagnostic Runner
- **Action**: Build client-side engine capable of running TR-143 diagnostic jobs against remote TR-143 test servers.
- **Vue 3 UI**: Add TR-143 ISP Benchmark page in `qiluo_vue/src/views/Network/TR143Diagnostic.vue`.

---

### 4.3 Definition of Done (DoD) — Milestone 4
- [ ] Standard HTTP clients (`curl`, browser, ONT CPE firmware) can execute download tests against `/diagnostics/tr143/download`.
- [ ] Upload endpoint handles continuous multi-gigabit streams without memory leakage.
- [ ] UDP Echo server mirrors packets with $< 100\mu\text{s}$ internal processing delay.
- [ ] Results produce TR-143 compliant metrics: `ROMTime`, `BOMTime`, `EOMTime`, `TestBytesReceived`, `Throughput`.

---

# Milestone 5: Deep Diagnostics (TCPDump & PCAP Management)

### 5.1 Objective
Provide strictly bounded, secure packet capture capabilities with BPF filter validation, artifact storage, and automated retention cleanup.

---

### 5.2 Step-by-Step Implementation Tasks

#### Task 5.1: BPF Validator & TCPDump Tool Engine
- **Action**: Create `qiluo_admin/src/tools/tcpdump.rs`.
- **Security Bounds**:
  - Maximum capture duration: $300\text{ seconds}$ (default $60s$).
  - Maximum packet count: $500,000\text{ packets}$.
  - Maximum file size: $250\text{MB}$.
- **BPF Validation**: Filter string validated against dangerous characters and validated via `libpcap` compiler before execution.

#### Task 5.2: Process Lifecycle & Graceful Termination
- **Implementation**:
  - Subprocess executed with `tokio::process::Command`.
  - User cancellation or timeout triggers `SIGINT` (to allow tcpdump to flush the PCAP buffer and exit cleanly).

#### Task 5.3: PCAP Artifact Storage & Auto-Cleanup Cron
- **Action**: Store PCAPs in `/var/log/r-speedtest/pcaps/<uuid>.pcap`.
- **Cleanup Worker**: Periodic job in `qiluo_admin/src/worker/pcap_cleanup_worker.rs` deleting PCAP files older than 48 hours.

#### Task 5.4: Authenticated PCAP Download API
- **Endpoint**: `GET /api/net/jobs/:id/pcap`.
- **Permission**: `net:tcpdump:download` (strictly restricted).

---

### 5.3 Definition of Done (DoD) — Milestone 5
- [ ] Authorized user can start packet capture on interface `eth0` with filter `port 443`.
- [ ] Stopping the capture or hitting timeout produces a valid `.pcap` file.
- [ ] Downloaded `.pcap` file opens cleanly in Wireshark with correct packet headers.
- [ ] Ephemeral retention cron successfully deletes captures older than 48 hours.

---

# Milestone 6: Scenario Engine & Automation DAG Workflows

### 6.1 Objective
Build an automated Diagnostic Scenario Engine capable of executing multiple tools in directed workflows (DAG) with conditional branching and link health scorecards.

---

### 6.2 Step-by-Step Implementation Tasks

#### Task 6.1: Scenario Definition Schema & Execution Flow
- **Action**: Create `qiluo_admin/src/service/s_net_scenario.rs` and `qiluo_admin/src/worker/scenario_worker.rs`.
- **Scenario DAG JSON Structure**:
  ```json
  {
    "steps": [
      { "step_id": "1", "tool": "ping", "target": "$GATEWAY", "on_failure": "abort" },
      { "step_id": "2", "tool": "dns", "target": "isp-core.net", "on_failure": "continue" },
      { "step_id": "3", "tool": "tr143_download", "target": "speedtest.isp.net", "depends_on": ["1"] },
      { "step_id": "4", "tool": "udpecho", "target": "speedtest.isp.net", "depends_on": ["1"] }
    ]
  }
  ```

#### Task 6.2: Scorecard Calculation & Health Algorithm
- **Action**: Calculate weighted score (0 - 100%) based on:
  - Latency SLA ($20\%$), Packet Loss ($30\%$), Throughput SLA ($30\%$), Jitter SLA ($20\%$).

#### Task 6.3: Vue 3 Scenario Builder & Report View
- **Action**: Create `qiluo_vue/src/views/Network/ScenarioBuilder.vue` and `ScenarioRunner.vue`.
- **Features**: Drag-and-drop or step list builder, real-time workflow DAG execution indicator, and one-click PDF/JSON scorecard export.

---

### 6.3 Definition of Done (DoD) — Milestone 6
- [ ] Multi-step "Customer ONT Diagnostic" scenario executes all 4 steps sequentially/in parallel.
- [ ] If Step 1 (Ping Gateway) fails, execution halts immediately with "Gateway Unreachable" error.
- [ ] Generates an aggregated diagnostic scorecard with health score and exportable PDF report.

---

# Milestone 7: Distributed Edge Probe Multi-Node Coordination

### 7.1 Objective
Enable multi-region distributed diagnostics where a Central Controller dispatches diagnostic jobs to remote Edge Node Agents (e.g., Jakarta, Bandung, Pontianak).

---

### 7.2 Step-by-Step Implementation Tasks

#### Task 7.1: Node Agent Architecture
- **Action**: Create lightweight agent mode inside `qiluo_admin` (`--node-agent`).
- **Communication**: Edge Node opens an encrypted outbound WebSocket connection to the Central Controller (`wss://controller/api/net/node-gateway`).
- **Authentication**: HMAC SHA-256 Token / Ed25519 preshared key.

#### Task 7.2: Node Heartbeat & Status Monitor
- **Action**: Create `qiluo_admin/src/worker/node_heartbeat_worker.rs`.
- **Metrics**: Reports CPU load, RAM usage, interface bandwidth, and active worker count every 10 seconds.
- **Node States**: `online`, `degraded` (high load), `offline` (missed 3 heartbeats), `maintenance`.

#### Task 7.3: Remote Job Router & Vue 3 Node Map
- **Action**: When creating a Job, user selects Source Node: `[x] Bandung POP [ ] Jakarta POP`.
- **Vue 3 UI**: Add Node Topology Management in `qiluo_vue/src/views/Network/NodeManagement.vue`.

---

### 7.3 Definition of Done (DoD) — Milestone 7
- [ ] Edge Node in a remote network registers with Controller over outbound WebSocket without opening inbound firewall ports.
- [ ] Controller dispatches an iPerf3 job to the remote node; the node executes the test and streams telemetry back to the central UI.
- [ ] Node disconnect marks status as `offline` within 30 seconds.

---

# Milestone 8: Historical Analytics, Alerting, NOC Dashboard & Polish

### 8.1 Objective
Build the unified 24/7 NOC wallboard, historical trend analytics, multi-channel alerting (Telegram, Discord, Slack, Email), and prepare production release packages.

---

### 8.2 Step-by-Step Implementation Tasks

#### Task 8.1: Historical Time-Series Analytics & ECharts
- **Action**: Create `qiluo_vue/src/views/Network/Analytics.vue`.
- **Visualizations**: Latency trends over 24h/7d/30d, ISP Bandwidth SLA delivery rate, Packet Loss heatmap by region.

#### Task 8.2: NOC Alert Notification Engine
- **Action**: Create `qiluo_admin/src/service/s_net_alert.rs`.
- **Channels**: Webhook, Telegram Bot, Discord Webhook, Email (via QiLuo’s `MailerWorker`).
- **Triggers**: Diagnostic test failure, TR-143 throughput below SLA threshold, Edge Node offline.

#### Task 8.3: Unified 24/7 NOC Operations Wallboard
- **Action**: Create `qiluo_vue/src/views/Dashboard/NocWallboard.vue`.
- **Design**: High-contrast dark NOC theme, real-time active job counters, live throughput speedometer, alert stream, and active node status cards.

#### Task 8.4: Tauri Desktop Packaging & Multi-Arch Docker
- **Action**: Configure `qiluo_tauri` build scripts and test Linux AppImage/deb, Windows msi, and macOS dmg packages.

---

### 8.3 Definition of Done (DoD) — Milestone 8
- [ ] 24/7 NOC Wallboard runs smoothly on a big-screen display with live WebSocket metrics.
- [ ] SLA breach triggers instant Telegram / Webhook notification with test summary.
- [ ] `docker compose up -d` boots the entire production stack in $< 3\text{ minutes}$.
- [ ] Tauri desktop application builds and connects seamlessly to local or remote servers.
