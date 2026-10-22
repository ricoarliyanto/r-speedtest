# Production Deployment & Infrastructure Guide (DEPLOYMENT)
## Network Swiss Army & Speedtest Diagnostic Platform

---

## 1. Production Topology & Overview

The production architecture consists of containerized microservices managed via **Docker Compose** or **Kubernetes**, ensuring high availability, process isolation, and multi-gigabit throughput capabilities.

```
                          [ Client / Browser / Desktop ]
                                        |
                                        v
                       +---------------------------------+
                       |   Reverse Proxy (Caddy / NGINX) |
                       |   - TLS Termination             |
                       |   - Static Assets Caching       |
                       |   - WebSocket WSS Upgrade       |
                       +----------------+----------------+
                                        |
                                        v
                       +---------------------------------+
                       |   QiLuo Admin Backend (Rust)    |
                       |   - Port: 3000                  |
                       |   - REST API & WebSockets       |
                       +-------+-----------------+-------+
                               |                 |
                SQL Persistent |                 | Redis API (Queue/Events)
                               v                 v
           +-------------------+---+         +---+-------------------+
           |      PostgreSQL 16    |         |    DragonflyDB        |
           |      Port: 5432       |         |    Port: 6379         |
           +-----------------------+         +---+-------------------+
                                                 |
                                                 | BRPOP Task Queue
                                                 v
                                     +-----------+-------------------+
                                     | Network Diagnostic Worker     |
                                     | - Ping, MTR, Traceroute       |
                                     | - iPerf3, TR-143, TCPDump     |
                                     | - Cap: NET_RAW, NET_ADMIN     |
                                     +-------------------------------+
```

---

## 2. Docker Compose Configuration (`docker-compose.yml`)

Save this file in the project root:

```yaml
version: "3.8"

services:
  # 1. Primary PostgreSQL Persistent Database
  postgres:
    image: postgres:16-alpine
    container_name: rspeedtest-postgres
    restart: unless-stopped
    environment:
      POSTGRES_USER: qiluo
      POSTGRES_PASSWORD: qiluo_secure_password_2026
      POSTGRES_DB: qiluo_net
      PGDATA: /var/lib/postgresql/data/pgdata
    volumes:
      - pgdata:/var/lib/postgresql/data
      - ./qiluo_admin/qiluo_pg.sql:/docker-entrypoint-initdb.d/init.sql:ro
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U qiluo -d qiluo_net"]
      interval: 5s
      timeout: 5s
      retries: 5
    networks:
      - rspeedtest-net

  # 2. DragonflyDB - Ultra-High-Performance Redis Alternative
  dragonfly:
    image: docker.dragonflydb.io/dragonflydb/dragonfly:latest
    container_name: rspeedtest-dragonfly
    restart: unless-stopped
    ulimits:
      memlock: -1
      nofile:
        soft: 65536
        hard: 65536
    ports:
      - "6379:6379"
    volumes:
      - dragonflydata:/data
    command:
      - "--logtostderr"
      - "--dir=/data"
      - "--maxmemory=2GB"
    networks:
      - rspeedtest-net

  # 3. QiLuo Application Server (Web API & Dashboard)
  app:
    build:
      context: .
      dockerfile: Dockerfile
      target: runner
    container_name: rspeedtest-app
    restart: unless-stopped
    depends_on:
      postgres:
        condition: service_healthy
      dragonfly:
        condition: service_started
    environment:
      - RUST_LOG=info,qiluo=debug
      - DATABASE_URL=postgres://qiluo:qiluo_secure_password_2026@postgres:5432/qiluo_net
      - REDIS_URL=redis://dragonfly:6379/
      - APP_PORT=3000
    ports:
      - "3000:3000"
    volumes:
      - pcap_data:/var/log/r-speedtest/pcaps
    networks:
      - rspeedtest-net

  # 4. Dedicated Network Diagnostic Worker Container
  worker:
    build:
      context: .
      dockerfile: Dockerfile
      target: runner
    container_name: rspeedtest-worker
    restart: unless-stopped
    depends_on:
      postgres:
        condition: service_healthy
      dragonfly:
        condition: service_started
    environment:
      - RUST_LOG=info,qiluo=debug
      - DATABASE_URL=postgres://qiluo:qiluo_secure_password_2026@postgres:5432/qiluo_net
      - REDIS_URL=redis://dragonfly:6379/
      - IS_WORKER_ONLY=true
    cap_add:
      - NET_RAW
      - NET_ADMIN
    volumes:
      - pcap_data:/var/log/r-speedtest/pcaps
    networks:
      - rspeedtest-net

  # 5. Reverse Proxy with Auto-HTTPS
  caddy:
    image: caddy:2-alpine
    container_name: rspeedtest-caddy
    restart: unless-stopped
    ports:
      - "8030:8030"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile:ro
      - caddy_data:/data
      - caddy_config:/config
    depends_on:
      - app
    networks:
      - rspeedtest-net

volumes:
  pgdata:
  dragonflydata:
  pcap_data:
  caddy_data:
  caddy_config:

networks:
  rspeedtest-net:
    driver: bridge
```

---

## 3. Multi-Stage Production Dockerfile (`Dockerfile`)

```dockerfile
# -----------------------------------------------------------------------------
# Stage 1: Build Vue 3 Frontend
# -----------------------------------------------------------------------------
FROM node:20-alpine AS frontend-builder
WORKDIR /app/qiluo_vue
RUN npm install -g pnpm
COPY qiluo_vue/package.json qiluo_vue/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY qiluo_vue/ ./
RUN pnpm build:pro

# -----------------------------------------------------------------------------
# Stage 2: Build Rust Backend
# -----------------------------------------------------------------------------
FROM rust:1.80-bullseye AS backend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpcap-dev

COPY qiluo_admin/Cargo.toml qiluo_admin/Cargo.lock ./qiluo_admin/
COPY qiluo_admin/migration/Cargo.toml ./qiluo_admin/migration/
# Create dummy main to cache dependencies
RUN mkdir -p qiluo_admin/src qiluo_admin/migration/src && \
    echo "fn main() {}" > qiluo_admin/src/main.rs && \
    echo "fn main() {}" > qiluo_admin/migration/src/main.rs
WORKDIR /app/qiluo_admin
RUN cargo build --release || true

COPY qiluo_admin/ ./
RUN cargo build --release

# -----------------------------------------------------------------------------
# Stage 3: Runtime Container with Network Utilities
# -----------------------------------------------------------------------------
FROM debian:bullseye-slim AS runner
WORKDIR /app

# Install system network diagnostic tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    iputils-ping \
    traceroute \
    mtr-tiny \
    iperf3 \
    tcpdump \
    dnsutils \
    curl \
    libpcap0.8 \
    libssl1.1 \
    libcap2-bin && \
    rm -rf /var/lib/apt/lists/*

# Grant non-root raw socket capabilities to ping and tcpdump if needed
RUN setcap cap_net_raw+ep /usr/bin/iperf3 || true
RUN setcap cap_net_raw,cap_net_admin+ep /usr/bin/tcpdump || true

# Copy backend binary and config
COPY --from=backend-builder /app/qiluo_admin/target/release/qiluo /app/qiluo
COPY --from=backend-builder /app/qiluo_admin/config /app/config

# Copy compiled frontend dist into static web directory
COPY --from=frontend-builder /app/qiluo_vue/dist /app/dist

# Create pcap directory
RUN mkdir -p /var/log/r-speedtest/pcaps

EXPOSE 3000 37001/udp 5201

CMD ["/app/qiluo"]
```

---

## 4. Reverse Proxy Configuration (`Caddyfile`)

```caddyfile
:8030 {
    # Replace with your domain in production (e.g. speedtest.example.com)
    encode zstd gzip

    # Frontend Static SPA Routing
    root * /app/dist
    file_server

    # WebSocket Telemetry Proxy
    @ws {
        header Connection *Upgrade*
        header Upgrade    websocket
        path /api/net/ws*
    }
    reverse_proxy @ws app:3000

    # REST API Proxy
    handle /api/* {
        reverse_proxy app:3000
    }

    # SPA Fallback
    try_files {path} /index.html
}
```

---

## 5. Host Linux Kernel & Network Tuning (`/etc/sysctl.d/99-speedtest.conf`)

For dedicated bare-metal or high-throughput cloud VMs ($10\text{Gbps}+$ line rate testing):

```ini
# Maximum socket receive and send buffer sizes
net.core.rmem_max = 67108864
net.core.wmem_max = 67108864
net.core.rmem_default = 33554432
net.core.wmem_default = 33554432

# TCP buffer limits: min default max
net.ipv4.tcp_rmem = 4096 87380 67108864
net.ipv4.tcp_wmem = 4096 65536 67108864

# Maximum network device backlog queue
net.core.netdev_max_backlog = 250000

# Enable modern TCP Congestion Control (BBR)
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr

# Fast socket recycling and port range
net.ipv4.ip_local_port_range = 1024 65535
net.ipv4.tcp_tw_reuse = 1
net.ipv4.tcp_fin_timeout = 15
```

Apply immediately on host:
```bash
sudo sysctl -p /etc/sysctl.d/99-speedtest.conf
```

---

## 6. Deployment Verification & Health Check

After launching containers (`docker compose up -d`), verify all components:

```bash
# 1. Check container health status
docker compose ps

# 2. Test DragonflyDB connectivity
docker compose exec dragonfly dragonfly-cli ping
# Output: PONG

# 3. Test PostgreSQL connectivity
docker compose exec postgres pg_isready -U qiluo -d qiluo_net
# Output: /var/run/postgresql:5432 - accepting connections

# 4. Check Backend API health
curl -s http://localhost:3000/api/health
```
