# Walkthrough: Milestone 6 — Composite Diagnostic Scenarios (DAG Workflow Engine)

Milestone 6 has been implemented and verified (**Exit Code: 0**). Multi-step diagnostic DAG workflows, step dependency execution, automated failure handling (`stop_on_failure` / `on_failure`), real-time event broadcasting, and composite Network Health Score calculation ($0 \sim 100$) are now active.

---

## Changes Implemented in Milestone 6

### 1. Scenario Execution Worker Engine (`src/worker/net_scenario_worker.rs`)
- **[`src/worker/net_scenario_worker.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/net_scenario_worker.rs)**:
  - Consumes execution messages from DragonflyDB queue `queue:net_scenario`.
  - Dynamically runs sequential/DAG step pipelines (`Ping` $\to$ `DNS` $\to$ `PortCheck` $\to$ `iPerf3` $\to$ `TR-143` $\to$ `TCPDump`).
  - Records child `net_job` and `net_job_result` entries.
  - Computes composite Network Health Score ($0 \sim 100$) based on packet loss %, latency deviations, DNS lookup speed, and throughput SLA breaches.
  - Broadcasts `scenario_finished` events to DragonflyDB cache key `scenario_result:<execution_id>`.
- **[`src/worker/processor_manager.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/processor_manager.rs)**: Registered `NetScenarioWorker` into the background processor.

### 2. Scenario Management & Execution Services (`src/service/net/s_net_scenario.rs`)
- **[`src/service/net/s_net_scenario.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/service/net/s_net_scenario.rs)**:
  - `create_scenario`: Creates reusable multi-step scenario templates.
  - `edit_scenario`: Modifies existing workflow templates.
  - `list_scenarios`: Paginated scenario query.
  - `get_scenario`: Fetches scenario details and JSON step definitions.
  - `delete_scenario`: Removes scenario templates.
  - `run_scenario`: Instantiates scenario execution against a target host or remote edge node.

### 3. Scenario REST API Routing (`src/api/net_scenario_api.rs`)
- **[`src/api/net_scenario_api.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/api/net_scenario_api.rs)**:
  - Registered `/net/scenario/create`, `/net/scenario/edit`, `/net/scenario/list`, `/net/scenario/get`, `/net/scenario/delete`, `/net/scenario/run`.
- **[`src/api.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/api.rs)**: Merged scenario router into QiLuo `WebApi`.

---

## Next Milestone

Ready to proceed with **Milestone 7: Distributed Edge Probes & Multi-Node Orchestration** (Agent token authentication, heartbeat collector, distributed job dispatching via DragonflyDB queues, and node health monitoring).
