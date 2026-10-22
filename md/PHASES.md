# Phased Implementation Roadmap (PHASES)
## Network Swiss Army & Speedtest Diagnostic Platform

---

## 1. Implementation Strategy & Principles

The development roadmap is structured into **8 iterative milestones** following the core philosophy:
> *"Build the foundation first, ensure bulletproof process isolation & security, then progressively deliver speedtest, ISP diagnostics, scenario automation, and distributed edge nodes."*

```mermaid
gantt
    title Network Swiss Army & Speedtest Implementation Roadmap
    dateFormat  YYYY-MM-DD
    section Core Foundation
    M1: Core Foundation & QiLuo Extension        :m1, 2026-09-01, 10d
    M2: Tool Engine & Realtime WS (Ping/Trace)   :m2, after m1, 12d
    section High-Speed Performance
    M3: iPerf3 Engine & UDP Echo Tester          :m3, after m2, 14d
    M4: TR-143 ISP Diagnostic Server             :m4, after m3, 14d
    section Deep Diagnostics & Automation
    M5: TCPDump & Safe PCAP Management           :m5, after m4, 10d
    M6: Scenario Diagnostic DAG Engine           :m6, after m5, 14d
    section Distributed Scaling & NOC
    M7: Distributed Edge Probe Multi-Node        :m7, after m6, 14d
    M8: Observability, Alerts & NOC Dashboard    :m8, after m7, 10d
```

---

## 2. Milestone Breakdown

---

### Milestone 1: Core Foundation & QiLuo Extension
**Objective**: Prepare the baseline QiLuo repository, database entities, DragonflyDB queue integration, and base UI routes.

#### Key Deliverables:
1. **Database Schema**:
   - Create Sea-ORM migration `m20260822_create_net_tables.rs` (`net_tools`, `net_jobs`, `net_job_results`, `net_nodes`, `net_scenarios`).
   - Run and verify migrations against PostgreSQL and SQLite.
2. **DragonflyDB Queue Registration**:
   - Register `queue:net_tool` and `queue:scenario` in `qiluo_admin/src/worker/processor_manager.rs`.
3. **RBAC Permissions**:
   - Seed granular permissions: `net:tool:execute`, `net:job:query`, `net:scenario:run`, `net:tcpdump:execute`.
4. **Vue 3 Navigation Layout**:
   - Register the `Network Tools`, `Speedtest`, `Scenarios`, and `Nodes` menus inside `qiluo_vue/src/router/routes/`.

**Definition of Done (DoD)**:
- Backend starts with PostgreSQL + DragonflyDB connected.
- New database tables exist and are queryable.
- Admin UI displays Network Diagnostic menu items.

---

### Milestone 2: Tool Registry, Job Engine & Real-Time Telemetry
**Objective**: Build the safe execution engine (`NetworkTool` trait), job queue processor, and WebSocket telemetry distribution for standard connectivity tools.

#### Key Deliverables:
1. **Tool Engine Core (`src/tools/`)**:
   - Implement `NetworkTool` trait and dynamic tool registry.
   - Implement **Ping Tool** (IPv4/IPv6, count, size, interval, latency parser).
   - Implement **Traceroute Tool** (hop-by-hop parser).
   - Implement **DNS / Port Check Tool**.
2. **Job Processor Worker (`src/worker/net_tool_worker.rs`)**:
   - Non-blocking Tokio subprocess execution with strict timeouts and argument whitelisting.
3. **WebSocket Hub (`src/api/net_ws_api.rs`)**:
   - Full-duplex WebSocket handler streaming line-by-line output and metrics to Vue 3.
4. **Vue 3 Tool Runner UI**:
   - Interactive diagnostic console with real-time ping RTT sparklines and hop tables.

**Definition of Done (DoD)**:
- Submitting a Ping or Traceroute via REST returns `202 Accepted` immediately.
- Live progress renders smoothly in Vue 3 via WebSocket.
- Execution history is persisted to `net_jobs` and `net_job_results` with audit log entries.

---

### Milestone 3: High-Throughput Speedtest Engine (iPerf3 & UDP Echo)
**Objective**: Implement carrier-grade throughput testing capable of multi-gigabit TCP/UDP rate measurement.

#### Key Deliverables:
1. **iPerf3 Integration (`src/tools/iperf3.rs`)**:
   - Server-mode lifecycle controller.
   - Client-mode executor with structured `-J` (JSON) streaming parser.
   - Metrics extracted: Bandwidth (Mbps/Gbps), Retransmissions, Congestion Window, Jitter, Loss%.
2. **UDP Echo Engine (`src/tools/udpecho.rs`)**:
   - High-precision microsecond packet sender & reflector for jitter calculation (RFC 3550).
3. **Speedtest Gauge & Charting Component (`qiluo_vue`)**:
   - Tachometer speedometer gauge and bandwidth vs. time ECharts integration.

**Definition of Done (DoD)**:
- Can execute 1Gbps / 10Gbps iPerf3 tests with realtime graph rendering.
- Handles reverse-mode (download vs upload) testing seamlessly.

---

### Milestone 4: ISP Standards (TR-143 Diagnostic Server)
**Objective**: Build native TR-143 compliant HTTP Download/Upload and Echo diagnostic engine for ISP/Telco ONT and ACS interoperability.

#### Key Deliverables:
1. **TR-143 HTTP Download Endpoint**:
   - Zero-copy chunked synthetic stream generation (`/diagnostics/tr143/download`).
   - Microsecond timing metrics (`BOMTime`, `EOMTime`, `ROMTime`, `TestBytesReceived`).
2. **TR-143 HTTP Upload Endpoint**:
   - High-throughput sink endpoint (`/diagnostics/tr143/upload`) measuring upload rate without disk write contention.
3. **TR-143 Diagnostic Engine Client**:
   - Client-side tester for automated CPE line testing.

**Definition of Done (DoD)**:
- Standard ONT/CPE or curl clients can run TR-143 download and upload tests against the server.
- Accurate metrics matching standard ISP speed benchmarks.

---

### Milestone 5: Deep Diagnostics (TCPDump & PCAP Management)
**Objective**: Safe, bounded packet capture for incident analysis with BPF validation.

#### Key Deliverables:
1. **TCPDump Tool Engine (`src/tools/tcpdump.rs`)**:
   - BPF filter syntax validator.
   - Hard limits: Max 250MB size, max 300s duration.
   - Non-blocking execution with explicit `SIGINT` termination.
2. **PCAP Storage & Download API**:
   - Secure authenticated one-time download tokens.
   - Automated 48-hour storage retention cleanup job.

**Definition of Done (DoD)**:
- Authorized Network Engineers can capture live packets on selected interfaces.
- Captured `.pcap` files can be downloaded directly from the UI for Wireshark inspection.

---

### Milestone 6: Scenario Engine & Automation Workflows
**Objective**: Orchestrate multiple diagnostic tools into an automated troubleshooting DAG (Directed Acyclic Graph).

#### Key Deliverables:
1. **Scenario Definition & Step Parser (`src/tools/scenario/`)**:
   - JSON-defined execution flow (Sequential / Parallel / Conditional branching).
2. **Scenario Execution Worker**:
   - Evaluates step conditions (e.g., *if Ping fails $\rightarrow$ abort with Gateway Alert; else proceed to Speedtest*).
3. **Scorecard Engine & PDF/JSON Export**:
   - Calculates aggregate health rating (0 - 100%) and generates diagnostic report.

**Definition of Done (DoD)**:
- NOC operator can trigger a one-click "ONT Health Check" running 5 tools in sequence and receive an instant health score and summary report.

---

### Milestone 7: Distributed Edge Probe Multi-Node Coordination
**Objective**: Support multi-region diagnostic execution (Jakarta, Bandung, Pontianak).

#### Key Deliverables:
1. **Node Agent Protocol**:
   - Outbound WebSocket connection from Edge Node to Central Controller with mTLS/HMAC token.
   - Heartbeat reporting status (`Online`, `Degraded`, `Offline`), CPU, and bandwidth.
2. **Remote Job Dispatcher**:
   - Route tool jobs to specific remote edge nodes.
3. **Multi-Node Topology Map in Vue 3**:
   - Map/card visualization showing active POPs and their diagnostic health.

**Definition of Done (DoD)**:
- User selects "Bandung POP" as source and tests reachability to an external target.

---

### Milestone 8: Historical Analytics, Alerting & Production Polish
**Objective**: Complete NOC dashboard, metric trends, Slack/Telegram notifications, and production release build.

#### Key Deliverables:
1. **Historical Trend Analytics**:
   - ECharts for latency over time, ISP bandwidth SLA trends, and packet loss distribution.
2. **NOC Alert Notification Engine**:
   - Webhook / Telegram / Email alerts on diagnostic failure or throughput degradation.
3. **Tauri Desktop & Docker Release**:
   - Multi-arch Docker images and signed desktop installers.

**Definition of Done (DoD)**:
- Full system passes automated integration tests.
- Production Docker compose deploys in $< 3\text{ minutes}$.

---

## 3. Risk Assessment & Mitigation Matrix

| Risk | Impact | Probability | Mitigation Strategy |
| :--- | :--- | :--- | :--- |
| **Command Injection Vulnerability** | Critical | Low | Strictly enforce `NetworkTool` trait with typed `Vec<String>` arguments; ban `sh -c` across the entire codebase. |
| **CPU Starvation during 10G Speedtests** | High | Medium | Isolate workers in separate processes/containers; use Tokio async I/O with zero-copy buffer allocations. |
| **Storage Exhaustion from PCAP Captures** | High | Low | Enforce hard 250MB limit per capture and automated 48-hour ephemeral file deletion worker. |
| **Node Agent Connection Drops** | Medium | Medium | Implement automatic reconnect with exponential backoff and persistent job state recovery in DragonflyDB. |
| **Long-Running Job Hangs** | Medium | Low | Hard process timeouts with automated `SIGKILL` escalation on unresponsive subprocesses. |
