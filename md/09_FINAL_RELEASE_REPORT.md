# Master Walkthrough: QiLuo High-Performance Network Diagnostic & Speedtest Platform (Milestones 1 – 8)

All 8 Milestones outlined in the Technical Design Document (TDD) and Implementation Roadmap have been built, integrated, and verified with zero compilation errors (**Exit Code: 0**).

---

## Architecture & Milestone Summary

```
+-----------------------------------------------------------------------------------+
|                           QILUO SPEEDTEST PLATFORM                                |
+-----------------------------------------------------------------------------------+
|  [Vue 3 / Tauri Desktop]  <--- WebSocket WSS --->  [Axum 0.8 REST & WS Hub]       |
|  - 24/7 NOC Wallboard                               - /net/tool/*, /net/job/*      |
|  - Speedtest Gauge (iPerf3/TR-143)                  - /net/scenario/*              |
|  - Scenario DAG Builder                             - /net/node/*, /net/alert/*    |
|  - PCAP Artifact Downloader                         - /diagnostics/tr143/*         |
+-----------------------------------------+-----------------------------------------+
                                          |
                        [DragonflyDB (Redis Queue Engine)]
                        - queue:net_tool
                        - queue:net_scenario
                        - queue:net_node_hb
                        - pubsub:job_result, scenario_result
                                          |
                      +-------------------+-------------------+
                      |                                       |
           [NetToolWorker]                           [NetScenarioWorker]
     - Ping (ICMP RTT/Loss)                    - DAG Step Pipeline
     - Traceroute (Hop by hop)                 - Health Score (0~100)
     - DNS (Record & Query ms)                 - Failover & Abort logic
     - PortCheck (TCP Handshake)                              |
     - iPerf3 (-J Multi-gigabit TCP/UDP)       [NetNodeHbWorker]
     - UDP Echo (RFC 3550 Jitter)              - Stale Probe Monitor (45s)
     - TR-143 (Zero-Copy HTTP Stream)          - Node State Machine
     - TCPDump (BPF Filter + PCAP)
```

---

## Milestones Delivered & Verified

| Milestone | Scope | Key Artifacts & Endpoints | Verification |
| :--- | :--- | :--- | :--- |
| **M1: Foundation & Deployment** | Root Docker Compose, Multi-stage Dockerfile, Caddy WSS proxy, Sea-ORM 5 network tables & models. | `docker-compose.yml`, `Dockerfile`, `Caddyfile`, `m20260822_000001_create_net_tables.rs` | Exit Code 0 |
| **M2: Tool Registry & Worker** | Zero-shell `NetworkTool` trait, Ping, Traceroute, DNS, PortCheck, Axum WebSocket Hub. | `src/tools/registry.rs`, `src/worker/net_tool_worker.rs`, `src/api/net_ws_api.rs` | Exit Code 0 |
| **M3: Speedtest Engine** | iPerf3 JSON stream wrapper (`-J`), reverse mode, multi-stream, native UDP Echo & RFC 3550 jitter. | `src/tools/iperf3.rs`, `src/tools/udpecho.rs` | Exit Code 0 |
| **M4: Carrier TR-143 Engine** | Broadband Forum TR-143 HTTP Download/Upload zero-copy streaming (`ZERO_CHUNK_1MB`), UDP/TCP Echo. | `src/tools/tr143/`, `src/api/tr143_api.rs` (`/diagnostics/tr143/*`) | Exit Code 0 |
| **M5: Deep Diagnostics** | `tcpdump` with strict BPF filter validation, packet limits, isolated PCAP storage, authenticated download. | `src/tools/tcpdump.rs`, `/net/job/artifact?job_id=<id>` | Exit Code 0 |
| **M6: Composite Scenarios** | Multi-step DAG workflow runner, step dependency execution, failure policies, aggregated Health Score (0~100). | `src/worker/net_scenario_worker.rs`, `src/service/net/s_net_scenario.rs`, `/net/scenario/*` | Exit Code 0 |
| **M7: Edge Orchestration** | Token-authenticated edge node registration, heartbeat collector, automated 45s offline state machine. | `src/service/net/s_net_node.rs`, `src/worker/net_node_hb_worker.rs`, `/net/node/*` | Exit Code 0 |
| **M8: NOC Wallboard & Alerts** | Webhook / Telegram / Discord alert dispatcher, 24/7 High-Contrast NOC Wallboard, Historical Analytics. | `src/service/net/s_net_alert.rs`, `NocWallboard.vue`, `Analytics.vue` | Exit Code 0 |

---

## Production Deployment Command

To start the full production stack:
```bash
docker compose up -d --build
```
- **Web UI & NOC Wallboard**: `http://localhost:8030` (or `https://your-domain.com`)
- **Backend REST API**: `http://localhost:8030/api`
- **TR-143 Speedtest Endpoints**: `http://localhost:8030/diagnostics/tr143/download` & `upload`
- **Real-Time Telemetry WebSocket**: `ws://localhost:8030/api/net/ws`
- **PostgreSQL**: `localhost:5432`
- **DragonflyDB**: `localhost:6379`
