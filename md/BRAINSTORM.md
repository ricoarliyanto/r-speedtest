# Brainstorming & Technical Deep Dives (BRAINSTORM)
## Network Swiss Army & Speedtest Diagnostic Platform

---

## 1. Executive Ideation & Core Themes

This document captures exploratory design discussions, edge cases, kernel performance optimizations, and future capabilities for transforming the QiLuo platform into the ultimate **Network Swiss Army & Speedtest Platform**.

```
                           +-------------------------------------+
                           |      INNOVATION HORIZONS            |
                           +------------------+------------------+
                                              |
     +-------------------+--------------------+--------------------+-------------------+
     |                   |                    |                    |                   |
     v                   v                    v                    v                   v
+---------+       +--------------+     +--------------+     +-------------+     +--------------+
| 10G+    |       | TR-143 / ACS |     | Scenario DAG |     | Distributed |     | eBPF / AI    |
| Tuning  |       | ONT Engine   |     | Orchestrator |     | Edge Probes |     | Diagnostics  |
+---------+       +--------------+     +--------------+     +-------------+     +--------------+
```

---

## 2. Multi-Gigabit (10G/25G/40G) Speedtest Performance Tuning

Achieving sustained 10Gbps+ throughput in user-space speedtests without dropping packets requires fine-tuning at the Linux kernel, socket, and memory allocation layers.

### 2.1 Kernel TCP Stack Tuning (BBR & Window Scaling)
- **Congestion Control**: Replace default `cubic` with `BBRv3` (Bottleneck Bandwidth and RTT) to maximize throughput over high-latency long-haul fiber links with non-congestive packet loss.
- **Socket Buffer Auto-Tuning**:
  ```ini
  # Linux sysctl optimization for 10Gbps line rates
  net.core.rmem_max = 67108864
  net.core.wmem_max = 67108864
  net.ipv4.tcp_rmem = 4096 87380 67108864
  net.ipv4.tcp_wmem = 4096 65536 67108864
  net.core.netdev_max_backlog = 250000
  net.ipv4.tcp_congestion_control = bbr
  net.core.default_qdisc = fq
  ```

### 2.2 Zero-Copy In-Memory Buffer Allocation
- When conducting TR-143 Download/Upload speedtests, never read files from disk.
- Allocate a shared, static 64MB circular buffer in memory initialized once at worker startup.
- Use `bytes::Bytes` with zero-copy slicing across HTTP/1.1 chunk streams or HTTP/2 frames.
- This ensures memory consumption remains static ($< 100\text{MB}$) regardless of how many multi-gigabit streams are running concurrently.

---

## 3. TR-143 / ACS / ONT ISP Ecosystem Integration

### 3.1 What is TR-143?
TR-143 is the Broadband Forum technical report defining Network-Side Diagnostic tests initiated by Customer Premises Equipment (CPE) or Auto-Configuration Servers (ACS) using TR-069 or TR-181 protocols.

### 3.2 Bridging QiLuo with ACS/ONT
```
  +-------------------------------------------------------+
  |                   ACS (GenieACS / OpenACS)           |
  +---------------------------+---------------------------+
                              | TR-069 RPC:
                              | SetParameterValues (DownloadDiagnostics)
                              v
  +-------------------------------------------------------+
  |              Customer ONT / CPE (Fiber Gateway)       |
  +---------------------------+---------------------------+
                              | HTTP GET (Chunked Test Payload)
                              | & UDP Echo (RFC 862)
                              v
  +-------------------------------------------------------+
  |           QiLuo TR-143 High-Speed Test Server         |
  |  - Endpoint: /diagnostics/tr143/download              |
  |  - Endpoint: /diagnostics/tr143/upload                |
  |  - UDP Echo Daemon: Port 37001                        |
  +-------------------------------------------------------+
```

### 3.3 Diagnostic Endpoints Design
1. `GET /diagnostics/tr143/download?size=104857600`: Streams 100MB of pre-allocated bytes, tracking exact client connection and completion timestamps.
2. `POST /diagnostics/tr143/upload`: Accepts chunked payload, discards bytes immediately in kernel buffer, calculates throughput without disk write overhead.
3. `UDPEchoServer`: Standalone Tokio UDP socket listening on port `37001` that mirrors incoming packets immediately with a hardware or monotonic clock timestamp.

---

## 4. Scenario Engine DAG & Decision Matrix

The Diagnostic Scenario Engine turns isolated tools into an automated troubleshooting pipeline.

### 4.1 Conditional Execution Graph
```
                          +-------------------+
                          | 1. Ping Gateway   |
                          +---------+---------+
                                    |
                    +---------------+---------------+
                    | Pass                          | Fail
                    v                               v
          +-------------------+           +-------------------+
          | 2. DNS Resolution |           | Alert: Gateway    |
          +---------+---------+           | Unreachable (STOP)|
                    |                     +-------------------+
            +-------+-------+
            | Pass          | Fail
            v               v
    +---------------+ +---------------+
    | 3. TR-143     | | Alert: DNS    |
    | Speedtest     | | Failure (STOP)|
    +-------+-------+ +---------------+
            |
            v
    +---------------+
    | 4. UDP Jitter |
    +-------+-------+
            |
            v
    +-----------------------------+
    | Aggregate Health Score: 95% |
    +-----------------------------+
```

### 4.2 Scorecard & Health Algorithm
Scenarios calculate an overall link quality rating:
- **Ping/RTT Weight (20%)**: RTT $< 10\text{ms} \rightarrow 100\text{ pts}$, RTT $> 50\text{ms} \rightarrow 50\text{ pts}$.
- **Packet Loss Weight (30%)**: Loss $= 0\% \rightarrow 100\text{ pts}$, Loss $> 2\% \rightarrow 0\text{ pts}$.
- **Throughput SLA Weight (30%)**: Actual Throughput vs Subscribed Plan SLA.
- **Jitter Weight (20%)**: Jitter $< 3\text{ms} \rightarrow 100\text{ pts}$.

---

## 5. Distributed Multi-Node Probe Network

### 5.1 Node Topologies
- **Central Controller**: The primary QiLuo admin dashboard & persistent PostgreSQL DB.
- **Edge Nodes**: Lightweight worker nodes running across regional POPs (e.g., Jakarta Datacenter, Bandung POP, Pontianak Access Node).

```
                            +--------------------+
                            | Central Controller |
                            |  (QiLuo Web UI)    |
                            +----+-----+----+----+
                                 |     |    |
           +---------------------+     |    +---------------------+
           | mTLS / WSS                |                          |
           v                           v                          v
+--------------------+      +--------------------+      +--------------------+
|  Jakarta POP Node  |      |  Bandung POP Node  |      | Pontianak POP Node |
|  - Worker Agent    |      |  - Worker Agent    |      |  - Worker Agent    |
|  - Local Tools     |      |  - Local Tools     |      |  - Local Tools     |
+--------------------+      +--------------------+      +--------------------+
```

### 5.2 Node Security & Authentication
- Nodes establish an encrypted outbound WebSocket connection (`wss://controller/api/net/node-gateway`) to avoid complex inbound firewall port forwarding.
- Authenticated via preshared Ed25519 token or mutual TLS certificates.
- Controller pushes diagnostic jobs down the WebSocket channel; Node streams metrics back.

---

## 6. Packet Analysis & PCAP Lifecycle

### 6.1 Safe BPF Filter Sanitization
- Users input Berkeley Packet Filter (BPF) strings (e.g., `host 10.0.0.1 and port 443`).
- Before execution, the filter string is passed through `libpcap`'s `pcap_compile()` validator in memory to reject syntax errors and prevent argument injection before calling the binary.

### 6.2 Ephemeral Storage & Retention
- Maximum capture: 250MB or 300 seconds.
- Files stored with randomized UUIDs in `/var/log/r-speedtest/pcaps/`.
- Automatic cron job deletes `.pcap` files older than 48 hours to maintain disk health.

---

## 7. Future Horizons: eBPF & AI Diagnostics

1. **eBPF Kernel Probes**: Future migration to `aya` (Rust eBPF framework) to capture TCP retransmits and round-trip times directly from the Linux kernel without running `tcpdump`.
2. **AI-Assisted Root-Cause Diagnostics**: Analyze combined traceroute, MTR, and jitter patterns to automatically classify network faults (e.g. *"Subsea Cable Degradation at Hop 5"*, *"Local Wi-Fi Channel Congestion"*, or *"Upstream Transit Packet Loss"*).
