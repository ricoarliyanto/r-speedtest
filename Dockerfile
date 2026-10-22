# -----------------------------------------------------------------------------
# Stage 1: Build Vue 3 Frontend
# -----------------------------------------------------------------------------
FROM node:22-bookworm-slim AS frontend-builder
WORKDIR /app/qiluo_vue
RUN npm install -g pnpm@9
COPY qiluo_vue/package.json ./
COPY qiluo_vue/pnpm-lock.yaml ./
RUN pnpm install --no-frozen-lockfile
COPY qiluo_vue/ ./
RUN pnpm build:pro

# -----------------------------------------------------------------------------
# Stage 2: Build Rust Backend
# -----------------------------------------------------------------------------
FROM rust:bookworm AS backend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpcap-dev

COPY qiluo_admin/ ./qiluo_admin/
WORKDIR /app/qiluo_admin
RUN cargo build --release

# -----------------------------------------------------------------------------
# Stage 3: Runtime Container with Network Utilities
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim AS runner
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
    libssl3 \
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
