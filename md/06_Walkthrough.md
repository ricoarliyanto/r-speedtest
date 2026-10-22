# Walkthrough: Milestone 5 — Deep Diagnostics (TCPDump & Safe PCAP Management)

Milestone 5 has been implemented and verified (**Exit Code: 0**). Deep packet capture diagnostics via `tcpdump`, strict BPF filter sanitization, artifact storage, and secure authenticated PCAP downloads are active.

---

## Changes Implemented in Milestone 5

### 1. TCPDump Packet Capture Tool (`src/tools/tcpdump.rs`)
- **[`src/tools/tcpdump.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/tcpdump.rs)**:
  - Strict BPF filter character allowlist (disallows shell operators `;`, `&`, `|`, `` ` ``, `$`, `<` `>`).
  - Interface sanitization and duration enforcement ($1 \sim 60\text{s}$).
  - Outputs captures to `./artifacts/pcap/<job_id>.pcap`.
  - Captures summary telemetry from stderr (packets captured, packets received by filter, packets dropped by kernel).

### 2. Registry & Worker Integration
- **[`src/tools/registry.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/registry.rs)**: Registered `TcpDumpTool` in `GLOBAL_TOOL_REGISTRY`.
- **[`src/worker/net_tool_worker.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/net_tool_worker.rs)**: Automatically initializes `./artifacts/pcap/` directory and sets `artifact_path` on the created `net_job_results` record.

### 3. Authenticated PCAP Download API
- **[`src/service/net/s_net_tool.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/service/net/s_net_tool.rs)** & **[`src/api/net_tool_api.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/api/net_tool_api.rs)**:
  - Route: `GET /net/job/artifact?job_id=<job_id>`.
  - Streams `.pcap` binary file with `Content-Type: application/vnd.tcpdump.pcap` and `Content-Disposition: attachment; filename="<job_id>.pcap"`.

---

## Next Milestone

Ready to proceed with **Milestone 6: Composite Diagnostic Scenarios (DAG Workflow Engine)** (DAG definition, step-by-step runner, conditional execution, and composite aggregated scoring).
