# Product Requirements Document (PRD)
## Network Swiss Army — Network Diagnostic & Speedtest Operations Platform

---

## 1. Executive Summary & Vision

### 1.1 Overview
The **Network Swiss Army & Speedtest Platform** is an enterprise-grade, fullstack network diagnostic, connectivity validation, throughput testing, packet analysis, and automated troubleshooting platform. Built on the **QiLuo** foundation (Rust + Axum + Sea-ORM + Vue 3 + TypeScript), it consolidates fragmented command-line networking tools into a unified, high-performance, secure, and auditable operational web & desktop suite.

### 1.2 Vision Statement
> *"To provide Network Engineers, NOC Operators, Telco/ISP Technicians, and Systems Administrators with an all-in-one 'Swiss Army Knife' for network diagnostics and speed measurement that deploys effortlessly, executes securely at multi-gigabit line rates, streams telemetry in real-time, and operates seamlessly across distributed edge nodes."*

---

## 2. Target Personas & Use Cases

### 2.1 User Personas

| Persona | Role | Key Needs & Pain Points | Primary Features Used |
| :--- | :--- | :--- | :--- |
| **NOC Operator** | 24/7 Monitoring & First-line Troubleshooting | Needs fast, standard diagnostic recipes without SSH access to production nodes. | Quick Ping/Traceroute, Diagnostic Scenarios, Alerting, Live Metrics |
| **Network / ISP Engineer** | Core & Access Network Optimization | Requires TR-143 ONT speed diagnostics, multi-stream iPerf3 throughput testing, and MTU/Jitter verification. | iPerf3, TR-143 Download/Upload, UDP Echo, Historical Trends |
| **Telco / Field Tech** | Customer Premise (ONT/CPE) Provisioning | Needs one-click validation of bandwidth SLA, DNS latency, and gateway reachability at install time. | One-Click Diagnostic Scenarios, Speedtest Engine, PDF/JSON Report Export |
| **SecOps / SysAdmin** | Incident Response & Deep Packet Inspection | Needs strictly bounded, auditable packet capture (tcpdump) without granting root shell access. | BPF Packet Capture, PCAP Download, Comprehensive Audit Trail, RBAC |

---

## 3. Product Goals & Core Tenets

1. **Leverage QiLuo Core**: Utilize QiLuo’s native Authentication, RBAC, Scheduler, Operation Logs, and UI layouts rather than reinventing foundational capabilities.
2. **Speed & Efficiency**: Built in Rust with async I/O (Tokio) and an in-memory queue/cache layer (DragonflyDB) to handle concurrent multi-gigabit throughput tests with minimal CPU overhead.
3. **Strict Security Posture**: Absolute zero arbitrary shell execution (`no sh -c`). Structured argument whitelisting, kernel capability isolation, output truncation, and strict timeouts.
4. **Everything is a Tool, Everything is a Job**: Unified lifecycle for all network actions (Submission $\rightarrow$ RBAC Check $\rightarrow$ Dragonfly Queue $\rightarrow$ Async Worker $\rightarrow$ DB Storage $\rightarrow$ WebSocket Broadcast $\rightarrow$ Audit Log).
5. **Distributed Edge Ready**: Architecture prepared for multi-node deployments (Controller managing Jakarta, Bandung, Pontianak edge probe workers).

---

## 4. Feature Specifications

### 4.1 Diagnostic & Connectivity Suite

#### 4.1.1 ICMP & High-Precision Ping
- **Target**: IPv4, IPv6, FQDN.
- **Parameters**: Packet count ($1 \sim 1000$), payload size ($32 \sim 65507$ bytes), interval ($0.1s \sim 10s$), timeout, DSCP/TOS tagging.
- **Output Metrics**: Transmitted/Received packets, Packet Loss %, Min/Avg/Max/StdDev latency, RTT distribution timeline.

#### 4.1.2 Traceroute & Hop-by-Hop Analysis
- **Target**: IPv4, IPv6, FQDN.
- **Parameters**: Max hops ($1 \sim 64$), probe count per hop, timeout, protocol selection (ICMP / UDP / TCP SYN).
- **Output Metrics**: Per-hop IP, ASN lookup, reverse DNS PTR, RTT per probe, loss detection per intermediate router.

#### 4.1.3 MTR (My Traceroute / Continuous Path Quality)
- **Target**: IPv4, IPv6, FQDN.
- **Parameters**: Cycle count, interval, payload size.
- **Output Metrics**: Real-time updating table of hops with Hostname, Loss%, Sent, Last RTT, Avg, Best, Worst, StDev.

#### 4.1.4 DNS Diagnostics
- **Target**: Query name, Record types (`A`, `AAAA`, `CNAME`, `MX`, `TXT`, `NS`, `SOA`, `PTR`, `SRV`).
- **Parameters**: Custom DNS server / resolver IP, port, DNSSEC flag, TCP/UDP fallback.
- **Output Metrics**: Query response time, Authoritative answer flag, TTL, Answer / Authority / Additional sections.

#### 4.1.5 Port & HTTP Service Checks
- **Port Check**: TCP 3-way handshake time, UDP service probing, banner grabbing (sanitized).
- **HTTP/HTTPS Check**: TLS handshake time, SSL/TLS Certificate expiration, HTTP status code, TTFB (Time to First Byte), Total download time, Content-length check.

---

### 4.2 Speedtest & High-Throughput Performance Suite

#### 4.2.1 iPerf3 Enterprise Engine
- **Modes**: Client mode (test against external or internal iPerf3 servers) and Controlled Server mode.
- **Protocols**: TCP (window size, congestion control algorithm: cubic/bbr), UDP (bandwidth limit, packet size).
- **Parameters**: Duration ($1 \sim 3600s$), Parallel Streams ($1 \sim 64$), Reverse mode (Server to Client download), Port, Zero-copy buffer option.
- **Output Metrics**: Real-time second-by-second throughput (Mbps/Gbps), Retransmits, TCP Congestion Window (CWND), Jitter (ms), UDP Packet Loss (%).

#### 4.2.2 TR-143 (Broadband Forum ISP Diagnostic Standard)
- **Purpose**: Carrier-grade network diagnostic compatible with TR-069 / TR-181 ONT & ACS ecosystems.
- **Test Modules**:
  1. **HTTP Download Diagnostics**: Multi-connection chunked download test measuring line rate throughput.
  2. **HTTP Upload Diagnostics**: Multi-connection POST/PUT upload test with sustained payload generation.
  3. **UDPEcho Client & Server**: RFC 862 / TR-143 UDP Echo for ultra-precise jitter and round-trip time analysis.
  4. **TCPEcho Client & Server**: Low-overhead TCP loopback diagnostic.
- **Output Metrics**: ROMTime, BOMTime, EOMTime, TotalBytesTransmitted, TotalBytesReceived, TestBytesTransmitted, TestBytesReceived, Throughput (bps), Jitter, Latency.

#### 4.2.3 High-Precision UDP Echo
- **Parameters**: Payload size, Packet count ($1 \sim 50000$), Packet rate (packets/sec), Port.
- **Output Metrics**: One-way/two-way latency, packet reordering, packet loss rate, jitter curve.

---

### 4.3 Packet Capture & Deep Troubleshooting

#### 4.3.1 TCPDump & BPF Packet Capture
- **Interface Selection**: Physical interfaces (`eth0`, `bond0`), VLAN sub-interfaces (`eth0.100`), Bridge/Veth.
- **Filtering**: BPF (Berkeley Packet Filter) syntax validator and sanitizer.
- **Bounds & Protection**:
  - Max duration limit ($10s \sim 300s$, default $60s$).
  - Max packet count cap ($100 \sim 500,000$ packets).
  - Max capture file size limit ($5\text{MB} \sim 250\text{MB}$).
- **Output & Artifacts**: Sanitized PCAP file generation, automatic retention/cleanup policy, packet count breakdown (TCP, UDP, ICMP, ARP), and secure one-time download link.

---

### 4.4 Scenario & Diagnostic Automation Engine

#### 4.4.1 Scenario Workflow Definition
Allows grouping individual tools into directed execution recipes.
- **Example Scenario**: *ONT Customer Quality Verification*:
  1. `Ping` Default Gateway (Pre-check) $\rightarrow$ *If fails, abort with Gateway Unreachable alert.*
  2. `DNS Lookup` ISP Core Resolver $\rightarrow$ *Measure resolve latency.*
  3. `Port Check` Port 80/443 Web Services.
  4. `TR-143 Download` Test (100MB chunk).
  5. `TR-143 Upload` Test (50MB chunk).
  6. `UDP Echo` (500 packets) $\rightarrow$ *Measure Jitter & Packet Loss.*
- **Execution Rules**: Sequential, Parallel steps, Stop on failure, Conditional branching, Configurable retries.
- **Reporting**: Aggregate Scorecard (Pass / Degraded / Fail) with consolidated PDF/JSON report.

---

### 4.5 Distributed Multi-Node Probe Architecture

#### 4.5.1 Edge Node Management
- **Node Topology**: Central Controller (QiLuo Core) with Distributed Node Agents deployed in different POPs (e.g., Jakarta Datacenter, Bandung POP, Pontianak Sub-station).
- **Node Heartbeat & Registration**: Nodes register via mTLS or HMAC Token, reporting status (`Online`, `Degraded`, `Offline`, `Maintenance`), CPU load, interface bandwidth, and active workers.
- **Source Selection**: Users can choose any active Node as the source agent to execute diagnostics towards target destinations.

---

## 5. Non-Functional Requirements (NFR)

### 5.1 Performance & Scalability
- **API Response Latency**: $< 50\text{ms}$ for non-executing metadata and query endpoints.
- **Real-Time Telemetry Latency**: $< 100\text{ms}$ WebSocket event broadcast latency from worker to UI.
- **Concurrent Tests**: Support up to 50 concurrent active diagnostic/speedtest workers per multi-core node without starving the Axum web server.
- **Throughput Capability**: Native Rust worker capable of saturating 10Gbps+ network interfaces when executing TR-143 and iPerf3 tests with zero-copy I/O.

### 5.2 Security & Compliance
- **Process Isolation**: All binary executions spawned via Tokio `Command` using pre-compiled argument vectors (no raw string interpolation).
- **Role-Based Access Control**:
  - `Super Admin`: System configuration, Node management, Full tool access.
  - `Network Engineer`: All diagnostic tools, iPerf3, TR-143, Scenarios, TCPDump (with audit).
  - `NOC Operator`: Standard ping, traceroute, DNS, predefined Scenarios, Job view.
  - `Viewer`: Read-only historical job results and analytics.
- **Audit Logging**: Every single tool launch records user ID, IP, target address, full validated argument list, duration, and execution result.

### 5.3 Reliability & Fault Tolerance
- **Graceful Job Termination**: SIGINT/SIGTERM cancellation propagation to subprocesses on user abort or timeout.
- **Worker Crash Recovery**: Stalled jobs detected via DragonflyDB heartbeat and marked as `Failed/Timeout` rather than hanging indefinitely.

---

## 6. Success Metrics & KPIs

| Metric | Target Goal |
| :--- | :--- |
| **Deployment Speed** | Under 3 minutes via single `docker compose up -d` |
| **Diagnostic Time Savings** | 70% reduction in NOC troubleshooting duration via Scenarios |
| **Realtime Telemetry Refresh** | $\ge 2\text{ Hz}$ live UI updates during speedtests |
| **Security Incident Rate** | 0 injection vulnerabilities by design |
| **System Resource Footprint** | $< 150\text{MB}$ RAM idle footprint for backend and worker |
