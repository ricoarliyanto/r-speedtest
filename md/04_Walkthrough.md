# Walkthrough: Milestone 3 — High-Throughput Speedtest Engine (iPerf3 & UDP Echo)

Milestone 3 has been implemented and compiled successfully (**Exit Code: 0**). Carrier-grade multi-gigabit throughput testing via iPerf3 and native microsecond UDP Echo testing are now integrated into the platform.

---

## Changes Implemented in Milestone 3

### 1. iPerf3 Throughput & Speedtest Engine (`src/tools/iperf3.rs`)
- **[`src/tools/iperf3.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/iperf3.rs)**:
  - Supports Client-Mode and Controlled Server-Mode.
  - Parameter configuration: Target, Port (5201), Duration ($1 \sim 300s$), Parallel Streams ($1 \sim 64$), Protocol (TCP/UDP), Bandwidth limit, Reverse Mode (`-R` for download testing).
  - JSON output streaming parser (`-J`): Extracts second-by-second throughput intervals, TCP retransmissions, CWND sizes, UDP jitter, and packet loss percentages.

### 2. Microsecond UDP Echo Diagnostic Engine (`src/tools/udpecho.rs`)
- **[`src/tools/udpecho.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/udpecho.rs)**:
  - Native asynchronous Tokio UDP socket runner (`execute_native_udp_test`).
  - RFC 3550 Jitter algorithm implementation with sequence tracking:
    $$J(i) = J(i-1) + \frac{|D(i-1, i)| - J(i-1)}{16}$$
  - Measures one-way / round-trip delay, min/avg/max/stddev RTT, and packet loss rate.

### 3. Registry & Worker Integration
- **[`src/tools/registry.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/tools/registry.rs)**: Registered `Iperf3Tool` and `UdpEchoTool` in `GLOBAL_TOOL_REGISTRY`.
- **[`src/worker/net_tool_worker.rs`](file:///home/win/code/r-speedtest/qiluo_admin/src/worker/net_tool_worker.rs)**: Added dynamic dispatch for native tools (`builtin:udpecho`) and external binary processes (`iperf3`).

---

## Next Milestone

Ready to proceed with **Milestone 4: Carrier-Grade ISP Diagnostics (Native TR-143 Engine)** (Broadband Forum HTTP Download/Upload speedtest endpoints with zero-copy in-memory buffer streaming and microsecond timing).
