# Walkthrough: Milestone 7 — Distributed Edge Probes & Multi-Node Orchestration

Milestone 7 has been implemented and verified (**Exit Code: 0**). Distributed multi-node agent registration, token authentication, periodic heartbeat collection, edge health state machine, and node management APIs are now active.

---

## Changes Implemented in Milestone 7

### 1. Node Management & Heartbeat Services (`src/service/net/s_net_node.rs`)
- **[`src/service/net/s_net_node.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/service/net/s_net_node.rs)**:
  - `register_node`: Registers an edge probe node, provisions a cryptographically hashed authentication token, and assigns capabilities (`ping`, `traceroute`, `dns`, `portcheck`, `iperf3`, `udpecho`, `tr143`).
  - `node_heartbeat`: Verifies probe token, records CPU/RAM utilization and active job count, updates `last_heartbeat`, and marks node status as `online`.
  - `list_nodes`: Queries registered edge nodes with dynamic online/offline computation ($45\text{s}$ heartbeat grace window).
  - `delete_node`: De-registers edge probe node.

### 2. Node Heartbeat & Health Worker (`src/worker/net_node_hb_worker.rs`)
- **[`src/worker/net_node_hb_worker.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/net_node_hb_worker.rs)**:
  - Background worker polling queue `queue:net_node_hb`.
  - Scans nodes and flags any node missing heartbeat $> 45\text{s}$ as `offline`.
  - Broadcasts `node_offline` events to DragonflyDB cache for instant UI wallboard alerts.
- **[`src/worker/processor_manager.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/processor_manager.rs)**: Registered `NetNodeHbWorker` in `processor_job()`.

### 3. REST API Routing & Controller (`src/api/net_node_api.rs`)
- **[`src/api/net_node_api.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/api/net_node_api.rs)**:
  - `POST /net/node/register`, `GET /net/node/list`, `POST /net/node/delete` registered in authenticated router.
  - `POST /net/node/heartbeat` registered in public router for seamless edge probe reporting.
- **[`src/api.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/api.rs)**: Merged into QiLuo `WebApi`.

---

## Next Milestone

Ready to proceed with **Milestone 8: 24/7 NOC Wallboard, Alerts & Production Release** (High-contrast 24/7 NOC Wallboard UI, SLA breach alert dispatching to Telegram/Discord/Email, and Tauri desktop integration).
