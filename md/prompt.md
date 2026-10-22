# Project: Network Swiss Army — Network Diagnostic & Operations Platform

## 1. Project Overview

Bangun sebuah aplikasi **fullstack Network Diagnostic & Operations Platform** menggunakan **QiLuo** sebagai application foundation.

QiLuo yang digunakan adalah framework/admin platform berbasis:

* Rust
* Axum
* Sea-ORM
* Vue 3
* TypeScript
* RBAC
* Scheduler
* Operation Logs
* System Monitoring
* Background/Worker capability

Referensi QiLuo:

https://www.qiluo.vip/en/

Repository:

https://github.com/chelunfu/qiluo_admin

Jangan membuat framework baru. Manfaatkan sebanyak mungkin fitur bawaan QiLuo dan extend secara modular.

Tujuan aplikasi adalah menjadi **"Swiss Army Knife" untuk Network Engineer, NOC, ISP, Telco, dan System Administrator**, yang menyediakan berbagai network diagnostic, connectivity, performance testing, packet analysis, monitoring, automation, dan troubleshooting tools dalam satu platform.

---

# 2. Core Technology Stack

Gunakan stack berikut:

## Backend

* Rust
* QiLuo
* Axum
* Tokio
* Sea-ORM
* Serde
* WebSocket
* Tracing

## Frontend

* Vue 3
* TypeScript
* QiLuo frontend components/layout
* Gunakan komponen bawaan QiLuo jika tersedia

## Database

Gunakan database yang didukung oleh QiLuo/Sea-ORM dan sesuai dengan deployment target.

Prioritaskan:

* PostgreSQL

Database utama digunakan untuk:

* users
* roles
* permissions
* tools
* jobs
* job parameters
* job results
* audit logs
* network nodes
* schedules
* diagnostic scenarios
* scenario executions
* system configuration
* reports
* historical data

## Cache / Queue / Realtime

Gunakan:

**DragonflyDB**

sebagai pengganti Redis.

DragonflyDB digunakan untuk:

* cache
* job queue
* temporary job state
* pub/sub
* WebSocket event distribution
* rate limiting
* session/cache jika diperlukan
* worker coordination

Jangan menjadikan DragonflyDB sebagai primary persistent database.

---

# 3. Architecture Philosophy

Gunakan arsitektur modular.

Jangan membuat setiap network tool menjadi logic yang hardcoded di controller/API.

Gunakan abstraction:

```text
Tool
    ↓
Job
    ↓
Worker
    ↓
Execution
    ↓
Result
    ↓
Database
    ↓
Audit Log
```

Setiap tool harus dapat ditambahkan tanpa perlu mengubah core application secara signifikan.

---

# 4. High-Level Architecture

Gunakan konsep:

```text
                         ┌─────────────────────┐
                         │      Vue 3 UI       │
                         │                     │
                         │ Dashboard           │
                         │ Network Tools       │
                         │ Jobs                │
                         │ Monitoring          │
                         │ Logs                │
                         │ Reports             │
                         │ Scenarios           │
                         └──────────┬──────────┘
                                    │
                              REST / WebSocket
                                    │
                         ┌──────────▼──────────┐
                         │    QiLuo Backend    │
                         │                     │
                         │ Rust + Axum         │
                         │ RBAC                │
                         │ API                 │
                         │ Tool Registry       │
                         │ Job Manager         │
                         │ Scheduler           │
                         │ Audit               │
                         └──────┬────────┬─────┘
                                │        │
                       ┌────────▼───┐ ┌──▼─────────────┐
                       │ PostgreSQL │ │ DragonflyDB    │
                       │            │ │                │
                       │ Persistent │ │ Cache          │
                       │ Data       │ │ Queue          │
                       │ Results    │ │ Pub/Sub        │
                       │ Audit      │ │ Job State      │
                       └────────────┘ └───────┬────────┘
                                             │
                                    ┌────────▼────────┐
                                    │ Network Worker   │
                                    │                  │
                                    │ ping             │
                                    │ traceroute       │
                                    │ mtr              │
                                    │ iperf3           │
                                    │ TR-143           │
                                    │ tcpdump          │
                                    │ udpecho          │
                                    │ DNS              │
                                    │ port check       │
                                    │ etc.             │
                                    └──────────────────┘
```

---

# 5. Tool Engine

Buat sebuah **Tool Registry**.

Contoh:

```text
tools/
├── ping
├── traceroute
├── mtr
├── iperf3
├── tr143
├── udpecho
├── tcpdump
├── dns
├── portcheck
├── httpcheck
├── arp
├── route
├── socket
└── interface
```

Setiap tool harus memiliki metadata:

```text
id
name
description
category
version
enabled
required_permission
input_schema
output_schema
binary
execution_type
timeout
```

Contoh:

```text
iperf3

name:
iPerf3

category:
Performance

binary:
/usr/bin/iperf3

input:
target
port
protocol
duration
parallel_streams
bandwidth

output:
bandwidth
jitter
packet_loss
retransmits
cpu_usage
raw_output
```

---

# 6. Security Requirement

Jangan pernah mengeksekusi arbitrary shell command dari user.

JANGAN menggunakan pola:

```rust
sh -c user_input
```

Semua tool harus memiliki executable dan argument builder yang sudah ditentukan.

Contoh:

```text
Tool:
iperf3

Binary:
/usr/bin/iperf3

Allowed arguments:
-c
-p
-t
-P
-u
-b
```

Input user harus:

1. validated
2. sanitized
3. converted menjadi structured arguments
4. executed menggunakan safe process API
5. memiliki timeout
6. memiliki resource limit jika memungkinkan

Tool execution harus memiliki:

* timeout
* cancellation
* maximum output size
* maximum execution time
* permission check
* audit logging

---

# 7. Job Engine

Semua eksekusi tool harus direpresentasikan sebagai Job.

Contoh:

```json
{
  "id": "job-123",
  "tool": "iperf3",
  "target": "10.10.10.20",
  "status": "running",
  "requested_by": "rico",
  "created_at": "...",
  "started_at": "...",
  "finished_at": "..."
}
```

Status:

```text
pending
queued
running
completed
failed
cancelled
timeout
```

Job lifecycle:

```text
API Request
    ↓
Permission Check
    ↓
Validate Input
    ↓
Create Job
    ↓
Push Queue
    ↓
Worker
    ↓
Execute Tool
    ↓
Collect Result
    ↓
Store Result
    ↓
Audit Log
    ↓
Publish Event
    ↓
WebSocket
    ↓
Vue UI
```

---

# 8. Worker Architecture

Gunakan worker/background execution.

API server tidak boleh blocking ketika menjalankan tool yang membutuhkan waktu lama.

Contoh:

```text
QiLuo API
    │
    ▼
DragonflyDB Queue
    │
    ├── Worker 01
    ├── Worker 02
    └── Worker 03
```

Worker mengambil Job dari DragonflyDB.

Worker kemudian:

```text
fetch job
validate
execute
capture stdout
capture stderr
parse result
update job
save result
publish event
```

---

# 9. Realtime Execution

Gunakan WebSocket untuk realtime status.

Contoh:

```text
iPerf3 Test

Target:
10.10.10.20

Duration:
30 sec

Status:
RUNNING

Progress:
████████████████░░░░ 80%

Current:
842 Mbps

Average:
835 Mbps

Peak:
941 Mbps
```

Worker mengirim event melalui DragonflyDB Pub/Sub.

Backend meneruskan event ke WebSocket.

Vue menampilkan update secara realtime.

---

# 10. Network Tools

Implementasikan secara bertahap.

## Phase 1

### Ping

Support:

* IPv4
* IPv6
* packet count
* packet size
* timeout
* interval

Result:

```text
packet_sent
packet_received
packet_loss
min_latency
avg_latency
max_latency
stddev
raw_output
```

---

## Phase 2

### Traceroute

Support:

* IPv4
* IPv6
* max hops
* timeout
* protocol jika memungkinkan

Parse setiap hop.

---

## Phase 3

### MTR

Support:

* target
* cycles
* interval

Result:

* hop
* hostname
* loss
* latency
* best
* avg
* worst

---

## Phase 4

### iPerf3

Support:

* TCP
* UDP
* client
* server
* duration
* port
* parallel streams
* bandwidth limit
* reverse mode
* IPv4
* IPv6

Parse result menjadi structured data.

Contoh:

```text
bandwidth
jitter
packet_loss
retransmits
bytes
duration
```

---

# 11. TR-143

TR-143 harus menjadi salah satu fitur utama aplikasi.

Implementasikan TR-143 compatible test infrastructure untuk:

* Download Diagnostics
* Upload Diagnostics
* TCP Echo
* UDP Echo

Expose metric seperti:

```text
Throughput
Latency
Bytes Sent
Bytes Received
Packet Loss
Jitter
Start Time
End Time
Duration
```

Desain TR-143 module supaya nantinya dapat diintegrasikan dengan ACS/ONT ecosystem.

Arsitektur:

```text
ONT
 │
 ▼
ACS
 │
 ▼
TR-143 Server
 │
 ▼
Diagnostic Engine
 │
 ▼
Job
 │
 ▼
Result
```

---

# 12. UDP Echo

Implementasikan UDP Echo server/client.

Support:

* IPv4
* IPv6
* port
* payload size
* packet count
* interval
* timeout

Result:

```text
packets_sent
packets_received
packet_loss
latency
jitter
```

---

# 13. TCPDump

TCPDump harus diperlakukan berbeda dari simple command tools karena merupakan long-running process.

Support:

* interface
* BPF filter
* packet count
* duration
* snaplen
* output file

Contoh:

```text
interface:
eth0

filter:
host 10.10.10.20 and port 443

duration:
60 sec
```

Jangan hanya menyimpan raw output.

Jika capture file dibuat:

```text
.pcap
```

simpan sebagai artifact/file reference.

Berikan kemampuan:

* start capture
* stop capture
* download capture
* metadata
* packet count
* size
* duration

Pastikan permission dan storage limit diterapkan.

---

# 14. Additional Network Tools

Setelah MVP stabil, tambahkan:

```text
DNS Lookup
Reverse DNS
Port Check
HTTP Check
ARP
Routing Table
Interface Statistics
Socket/Connection Information
IP Address Information
MAC Address Lookup
Subnet Calculator
CIDR Calculator
Wake-on-LAN
```

Kemudian:

```text
SNMP
HTTP Performance
TLS Check
Certificate Check
NTP
DHCP diagnostics
BGP diagnostics
```

---

# 15. Diagnostic Scenario Engine

Ini adalah fitur pembeda utama.

User dapat membuat scenario yang terdiri dari beberapa tools.

Contoh:

```text
Scenario:
ONT Connectivity Diagnostic
```

Steps:

```text
1. Ping Gateway
2. Ping ONT
3. DNS Lookup
4. TCP Port 443
5. TR-143 Download
6. TR-143 Upload
7. iPerf3
```

Scenario engine:

```text
Scenario
   ↓
Step 1
   ↓
Step 2
   ↓
Step 3
   ↓
Step 4
   ↓
...
   ↓
Final Result
```

Support:

* sequential execution
* parallel execution
* conditional execution
* retry
* timeout
* stop-on-failure
* continue-on-failure

---

# 16. Audit Logging

Semua aktivitas harus tercatat.

Minimal:

```text
user
action
resource
tool
job_id
source_ip
parameters
status
error
created_at
duration
```

Contoh:

```text
User:
rico

Action:
TOOL_EXECUTE

Tool:
iperf3

Target:
10.10.10.20

Source:
192.168.1.100

Status:
SUCCESS

Duration:
31.2 sec
```

Gunakan QiLuo operation logging jika sudah memenuhi kebutuhan.

Jangan membuat duplicate logging system jika fitur bawaan QiLuo dapat digunakan.

---

# 17. RBAC

Gunakan RBAC QiLuo.

Contoh role:

```text
Super Admin
Admin
Network Engineer
NOC Operator
Viewer
```

Permission granular:

```text
tool.ping.execute
tool.iperf3.execute
tool.tr143.execute
tool.tcpdump.execute

job.view
job.cancel

scenario.create
scenario.execute

node.manage

audit.view
```

Tool berbahaya seperti tcpdump harus memiliki permission khusus.

---

# 18. Network Nodes

Arsitektur harus dipersiapkan untuk distributed execution.

Contoh:

```text
                    Controller
                        │
          ┌─────────────┼─────────────┐
          │             │             │
       Jakarta       Bandung       Pontianak
         Node           Node           Node
          │             │             │
        iperf3        ping          tcpdump
        TR143         mtr           TR143
```

User dapat memilih source node:

```text
Source:
[x] Jakarta
[ ] Bandung
[ ] Pontianak

Target:
8.8.8.8

Tool:
Ping
```

Node harus melakukan registration/heartbeat.

Node status:

```text
online
offline
degraded
maintenance
```

---

# 19. Dashboard

Buat dashboard modern dengan tampilan NOC/Network Operations.

Display:

```text
Total Jobs
Running Jobs
Success Rate
Failed Jobs
Active Nodes
Tools Available
Recent Activity
```

Quick Tools:

```text
[ Ping ]
[ Traceroute ]
[ MTR ]
[ iPerf3 ]
[ TR-143 ]
[ UDP Echo ]
[ TCPDump ]
[ DNS ]
```

Recent Jobs:

```text
Time
User
Tool
Source
Target
Status
Duration
Result
```

---

# 20. Tool Execution UI

Setiap tool memiliki halaman:

```text
Tool Name
Description

Target
Parameters

Advanced Options

[ RUN ]
```

Saat running:

```text
Status
Progress
Realtime Output
Metrics
```

Setelah selesai:

```text
Summary
Metrics
Raw Output
Execution Details
Audit
```

---

# 21. Historical Analytics

Semua result yang relevan harus dapat dianalisis.

Contoh iPerf3:

```text
Throughput over time
Average throughput
Peak throughput
Jitter
Packet loss
Retransmission
```

TR-143:

```text
Download trend
Upload trend
Latency trend
Failure rate
```

Ping:

```text
Latency trend
Packet loss trend
Availability
```

Gunakan data historis untuk membuat graph.

---

# 22. API

Semua tool harus dapat dipanggil melalui API.

Contoh:

```text
POST /api/tools/ping/run
POST /api/tools/iperf3/run
POST /api/tools/tr143/run
POST /api/tools/udpecho/run
POST /api/tools/tcpdump/start
POST /api/tools/tcpdump/stop
```

Job:

```text
GET /api/jobs
GET /api/jobs/:id
POST /api/jobs/:id/cancel
```

Scenario:

```text
GET /api/scenarios
POST /api/scenarios
POST /api/scenarios/:id/run
```

Node:

```text
GET /api/nodes
POST /api/nodes
GET /api/nodes/:id/status
```

API harus konsisten dan mengikuti style QiLuo.

---

# 23. Scheduler

Manfaatkan scheduler QiLuo.

Support:

```text
Every 1 minute
Every 5 minutes
Every hour
Daily
Weekly
Cron expression
```

Contoh:

```text
Every 5 minutes:

Ping 10.10.10.1
Ping 10.10.10.2
HTTP check https://example.com
```

Hasil disimpan sebagai historical data.

Jika terjadi failure:

```text
Alert
```

---

# 24. Notification

Siapkan abstraction notification.

Future support:

```text
Email
Telegram
Discord
Webhook
Slack
Microsoft Teams
```

Contoh:

```text
TR143 Download
< 100 Mbps
```

trigger:

```text
ALERT
```

---

# 25. Observability

Aplikasi harus menggunakan structured logging.

Gunakan Rust tracing.

Metric aplikasi:

```text
API request count
API latency
job count
job execution time
worker utilization
queue depth
tool failures
database latency
DragonflyDB latency
```

Jika memungkinkan siapkan integrasi:

```text
OpenTelemetry
Prometheus
Grafana
```

---

# 26. Containerization

Sediakan Docker Compose untuk development.

Minimal:

```text
services:

  app
  database
  dragonfly
  worker
```

Jika worker dapat digabungkan dengan application process pada development, tetap desain architecture supaya production dapat dipisahkan.

---

# 27. Security

Prioritaskan security.

Requirement:

* RBAC
* authentication
* authorization
* input validation
* command injection protection
* process timeout
* output size limit
* file size limit
* rate limiting
* audit log
* TLS-ready
* secrets tidak boleh hardcoded
* sensitive parameters tidak disimpan dalam plaintext
* tcpdump restricted permission
* worker isolation jika memungkinkan

---

# 28. Development Principle

Jangan langsung membuat seluruh fitur.

Implementasikan bertahap.

## Milestone 1 — Foundation

```text
QiLuo
PostgreSQL
DragonflyDB
Authentication
RBAC
Dashboard
Docker Compose
```

## Milestone 2 — Tool Engine

```text
Tool Registry
Job Engine
Worker
Queue
Audit
WebSocket
```

Implementasi:

```text
Ping
Traceroute
```

## Milestone 3 — Performance

```text
iPerf3
UDP Echo
```

## Milestone 4 — ISP Diagnostics

```text
TR-143
```

## Milestone 5 — Packet Analysis

```text
TCPDump
```

## Milestone 6 — Scenario

```text
Diagnostic Scenario Engine
```

## Milestone 7 — Distributed Node

```text
Node Agent
Remote Worker
Node Management
```

## Milestone 8 — Monitoring & Analytics

```text
Historical Metrics
Charts
Alerts
Notifications
```

---

# 29. Code Quality

Gunakan prinsip:

* modular
* strongly typed
* async
* testable
* maintainable
* secure by default
* no unnecessary abstraction
* no duplicate functionality already provided by QiLuo

Backend harus mengikuti Rust best practices.

Hindari:

* unwrap() pada production path
* panic-based error handling
* arbitrary shell execution
* blocking operation pada async runtime
* hardcoded configuration
* duplicated RBAC/auth implementation

Gunakan proper error handling.

---

# 30. Testing

Buat:

### Unit Test

Untuk:

* tool input validation
* parser
* result normalization
* job state transition
* permission check

### Integration Test

Untuk:

```text
API
 ↓
DragonflyDB
 ↓
Worker
 ↓
Tool
 ↓
Database
```

### Security Test

Test:

* command injection
* invalid arguments
* unauthorized tool execution
* tcpdump permission
* path traversal
* oversized output
* timeout abuse

---

# 31. Important Design Decision

Gunakan prinsip:

> "Everything is a Tool, Everything runs as a Job, Everything is auditable."

Tool:

```text
What can the platform do?
```

Job:

```text
What is currently being executed?
```

Result:

```text
What happened?
```

Audit:

```text
Who did it?
```

Scenario:

```text
How do we combine multiple tools into one diagnostic workflow?
```

Node:

```text
Where is the test executed?
```

---

# 32. Expected Final Product

Produk akhir harus terasa seperti:

```text
                NETWORK SWISS ARMY
                       │
        ┌──────────────┼──────────────┐
        │              │              │
     Diagnose       Measure        Monitor
        │              │              │
      Ping           iPerf3          Jobs
      MTR            TR-143          Nodes
      DNS            UDP Echo        Alerts
      TCPDump        HTTP            Logs
      TCP Check
        │
        └──────────────┬──────────────┘
                       │
                 SCENARIO ENGINE
                       │
                       ▼
              Automated Diagnostics
```

Bukan hanya aplikasi untuk menjalankan command Linux, tetapi platform yang dapat digunakan oleh:

* Network Engineer
* NOC
* ISP
* Telco
* System Administrator
* Field Engineer
* IT Operations

---

# 33. First Implementation Task

Sebelum menulis kode dalam jumlah besar:

1. Inspect struktur QiLuo yang digunakan.
2. Identifikasi fitur bawaan:

   * authentication
   * RBAC
   * database
   * scheduler
   * operation log
   * background task
   * WebSocket
   * configuration
3. Jangan implementasikan ulang fitur yang sudah tersedia.
4. Tentukan extension point untuk Network Tool Engine.
5. Tentukan schema database.
6. Tentukan abstraction Tool/Job/Worker.
7. Tentukan integrasi DragonflyDB.
8. Buat architecture document.
9. Buat migration plan.
10. Baru mulai implementasi Milestone 1.

Jangan langsung membuat semua tool.

Mulai dari:

```text
QiLuo
+
PostgreSQL
+
DragonflyDB
+
Tool Registry
+
Job Engine
+
Worker
+
Audit
+
WebSocket
+
Ping
+
Traceroute
```

Setelah foundation tersebut stabil, baru lanjut ke iPerf3, TR-143, UDP Echo, TCPDump, dan Scenario Engine.

---

# Final Design Principle

Aplikasi harus dibangun dengan prinsip:

**QiLuo sebagai application foundation.**

**Rust sebagai secure network execution engine.**

**Vue sebagai operational interface.**

**PostgreSQL sebagai persistent source of truth.**

**DragonflyDB sebagai high-speed cache, queue, dan realtime layer.**

**Tools sebagai modular plugins.**

**Jobs sebagai execution abstraction.**

**Workers sebagai execution layer.**

**Scenarios sebagai automation layer.**

**Nodes sebagai distributed execution layer.**

**Audit sebagai historical accountability layer.**

Target akhirnya adalah sebuah:

> **Open-source, extensible, distributed Network Diagnostic & Operations Platform — sebuah "Swiss Army Knife" untuk Network Engineer, NOC, ISP, dan Telco.**
