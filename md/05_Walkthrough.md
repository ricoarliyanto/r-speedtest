# Walkthrough: Milestone 4 — Carrier-Grade ISP Diagnostics (Native TR-143 Engine)

Milestone 4 has been implemented and compiled successfully (**Exit Code: 0**). Carrier-grade TR-143 HTTP Download/Upload speedtest endpoints, microsecond metrics extraction (`BOMTime`, `EOMTime`, `ROMTime`, `TestBytesReceived`, `Throughput`), and UDP/TCP Echo reflection daemons are now active.

---

## Changes Implemented in Milestone 4

### 1. Native TR-143 Engine (`src/tools/tr143/`)
- **[`src/tools/tr143/download.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/tr143/download.rs)**:
  - `tr143_download_handler`: High-speed HTTP download endpoint streaming 1MB zero-copy memory chunks (`ZERO_CHUNK_1MB`).
  - `execute_tr143_download_client`: Client diagnostic runner spawning concurrent HTTP connections, tracking `ROMTime`, `BOMTime`, `EOMTime`, and measuring multi-gigabit throughput.
- **[`src/tools/tr143/upload.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/tr143/upload.rs)**:
  - `tr143_upload_handler`: HTTP upload sink consuming incoming data streams in kernel buffers without disk contention.
  - `execute_tr143_upload_client`: Client upload runner streaming in-memory payload to remote TR-143 test endpoints.
- **[`src/tools/tr143/echo.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/tr143/echo.rs)**:
  - Background task listening on UDP port `37001` (RFC 862 / TR-143 UDP Echo) and TCP port `37002`.
- **[`src/tools/tr143/tr143_tool.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/tr143/tr143_tool.rs)**:
  - Implements the `NetworkTool` trait for TR-143 download and upload tests.

### 2. Public Diagnostic API Endpoints (`src/api/tr143_api.rs`)
- **[`src/api/tr143_api.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/api/tr143_api.rs)**:
  - Routes `GET /diagnostics/tr143/download` and `POST /diagnostics/tr143/upload`.
  - Registered in `WebApi::white_routers()` for unauthenticated ONT/CPE firmware and automated speedtest execution.

### 3. Registry & Worker Integration
- **[`src/tools/registry.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/registry.rs)**: Registered `Tr143Tool` in `GLOBAL_TOOL_REGISTRY`.
- **[`src/worker/net_tool_worker.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/net_tool_worker.rs)**: Added native dispatch for `builtin:tr143` download and upload clients.

---

## Next Milestone

Ready to proceed with **Milestone 5: Deep Diagnostics (TCPDump & Safe PCAP Management)** (BPF filter validation, process lifecycle management, authenticated PCAP artifact download, and automated 48-hour retention cleanup).
