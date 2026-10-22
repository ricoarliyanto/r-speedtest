# Implementation Plan: QiLuo Network Swiss Army & Speedtest Server Platform

This plan outlines the architecture and execution roadmap for transforming the unified QiLuo admin repository (`qiluo_admin`, `qiluo_vue`, `qiluo_tauri`) into a high-performance **Network Swiss Army & Speedtest Diagnostic Platform** (covering iPerf3, TR-143, UDP Echo, Ping, MTR, Traceroute, TCPDump, and Diagnostic Scenarios) with automated deployment workflows.

---

## User Review Required

> [!IMPORTANT]
> **Key Architectural Alignments**:
> 1. **QiLuo Native Foundation**: We leverage QiLuo's existing Axum, Sea-ORM, JWT/RBAC, and Operation Log systems without re-inventing authentication or middleware.
> 2. **DragonflyDB Integration**: Used as high-throughput drop-in replacement for Redis (queues, ephemeral state, Pub/Sub event broadcasting, WebSocket coordination).
> 3. **Non-blocking Worker Engine**: Long-running tests (iPerf3, TR-143, TCPDump, MTR) are queued and executed by async worker processes; live metrics stream to Vue 3 UI via WebSocket.
> 4. **Strict Security Model**: Zero `sh -c` arbitrary execution. Whitelisted binaries, strongly-typed argument builders, strict timeouts, input sanitization, and audit trails.

---

## Deliverables Breakdown

We will generate 7 comprehensive, production-grade documentation and blueprint files in the `md/` directory:

1. **`md/PRD.md` (Product Requirements Document)**:
   - Vision, Target Personas (ISP, NOC, Telco, Network Engineer).
   - Functional & Non-Functional Requirements.
   - Tool Feature Matrix (Ping, MTR, Traceroute, iPerf3, TR-143 Download/Upload/Echo, UDP Echo, TCPDump, Scenario Engine, Distributed Node Agent).
   - User Journey, Role-Based Access Control (RBAC), and Audit Compliance.

2. **`md/TDD.md` (Technical Design Document)**:
   - System Architecture Diagram & Dataflow (API -> Dragonfly Queue -> Worker -> Tool Engine -> DB & WebSocket).
   - Rust Module Structure in `qiluo_admin` (`src/tools/`, `src/jobs/`, `src/worker/`, `src/ws/`, `src/nodes/`).
   - Sea-ORM Database Schema & Entity Definitions (`tools`, `jobs`, `job_results`, `nodes`, `scenarios`, `scenario_steps`).
   - Tool Abstraction (`ToolTrait`, `ArgumentBuilder`, `Parser`, `StreamCollector`).
   - WebSocket Protocol & Real-Time Event Dispatching.
   - TR-143 & iPerf3 Speedtest Engine Specifications.

3. **`md/ADR.md` (Architecture Decision Records)**:
   - ADR-001: Adopting QiLuo as Core Application Framework.
   - ADR-002: DragonflyDB as Primary In-Memory Broker & Cache.
   - ADR-003: Process Isolation & Whitelist-only Tool Execution (No arbitrary shell).
   - ADR-004: Decoupled Worker Queue Architecture vs In-Process Handlers.
   - ADR-005: WebSocket Streaming over SSE for Bi-directional Real-time Telemetry.
   - ADR-006: Sea-ORM Migration Strategy for Speedtest & Diagnostic Entities.

4. **`md/BRAINSTORM.md` (Ideation, Deep Dives & Edge Cases)**:
   - TR-143 ACS / ONT Integration Scenarios & ISP Workflow automation.
   - Speedtest High-Load Optimization (multi-gigabit buffer tuning, kernel socket flags, TCP BBR).
   - Distributed Edge Probe / Multi-region Node Agents (Jakarta, Bandung, Pontianak, Remote).
   - Scenario Engine DAG (Directed Acyclic Graph) conditional branching & fallback logic.
   - Packet capture (.pcap) storage lifecycle & automated BPF sanitization.

5. **`md/DEPLOYMENT.md` (Production & Docker Deployment Guide)**:
   - Production Docker Compose setup (QiLuo App, DragonflyDB, PostgreSQL, Network Worker).
   - Multi-arch Dockerfile (x86_64, aarch64) with native network tools (iperf3, iputils-ping, traceroute, mtr, tcpdump).
   - Linux Kernel & Network Tuning (`sysctl.conf` for 10Gbps+ speedtest performance).
   - Reverse proxy & TLS configuration (Caddy / Nginx) with WebSocket support.
   - CI/CD build scripts and automated release pipelines.

6. **`md/SETUP.md` (Developer & Local Environment Setup)**:
   - Prerequisites (Rust 1.80+, Node.js 20+, Docker / DragonflyDB, PostgreSQL).
   - Step-by-step Backend setup (`qiluo_admin`), database migrations, and YAML configuration.
   - Step-by-step Frontend setup (`qiluo_vue`) and Tauri desktop build (`qiluo_tauri`).
   - Mocking & running local speedtest workers.
   - Troubleshooting common dev issues (capabilities, permissions, port conflicts).

7. **`md/PHASES.md` (Implementation Roadmap & Phased Delivery)**:
   - Milestone 1: Core Foundation & QiLuo Extension Setup.
   - Milestone 2: Tool Registry, Job Queue & WebSocket Telemetry (Ping/Traceroute).
   - Milestone 3: High-Speed Speedtest Engine (iPerf3 & UDP Echo).
   - Milestone 4: ISP Standards (TR-143 Download/Upload/Echo Diagnostic Server).
   - Milestone 5: Deep Diagnostics (TCPDump & PCAP Management).
   - Milestone 6: Scenario Engine & Automation Workflows.
   - Milestone 7: Distributed Multi-Node Orchestration.
   - Milestone 8: Observability, Historical Analytics & NOC Alerting.

---

## Proposed Changes

### Documentation Files in `md/`

#### [NEW] [PRD.md](file:///home/win/code/r-speedtest/md/PRD.md)
Comprehensive Product Requirements Document covering platform features, speedtest metrics, ISP workflows, and user permissions.

#### [NEW] [TDD.md](file:///home/win/code/r-speedtest/md/TDD.md)
Detailed Technical Design Document covering Rust structs, Sea-ORM schemas, Worker pipeline, WebSocket events, and tool execution security.

#### [NEW] [ADR.md](file:///home/win/code/r-speedtest/md/ADR.md)
Architecture Decision Records explaining the rationale behind framework choices, database selections, and security boundaries.

#### [NEW] [BRAINSTORM.md](file:///home/win/code/r-speedtest/md/BRAINSTORM.md)
Brainstorming notes on TR-143 ISP integration, 10G speedtest performance tuning, distributed probe agents, and advanced scenario branching.

#### [NEW] [DEPLOYMENT.md](file:///home/win/code/r-speedtest/md/DEPLOYMENT.md)
Production deployment blueprint, docker-compose.yml configurations, Linux kernel tuning, and automated deployment scripts.

#### [NEW] [SETUP.md](file:///home/win/code/r-speedtest/md/SETUP.md)
Local setup guide for developers covering backend, frontend, database, DragonflyDB, and Tauri.

#### [NEW] [PHASES.md](file:///home/win/code/r-speedtest/md/PHASES.md)
Phased milestone execution breakdown from Milestone 1 (Foundation) to Milestone 8 (Distributed NOC & Analytics).

---

## Verification Plan

### Automated Verification
- Verify that all generated markdown files exist, are properly structured with tables, code blocks, mermaid diagrams, and consistent linking.
- Validate formatting and markdown linting across all files in `md/`.

### Manual Verification
- Review technical alignment with existing `qiluo_admin`, `qiluo_vue`, and `qiluo_tauri` code structures.
