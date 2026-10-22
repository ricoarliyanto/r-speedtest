# Local Development & Setup Guide (SETUP)
## Network Swiss Army & Speedtest Diagnostic Platform

---

## 1. Prerequisites & System Requirements

Before setting up the project locally, ensure you have the following tools installed on your development machine:

### 1.1 Core Toolchains
- **Rust**: Version $\ge 1.80$ (with `cargo`, `rustc`, `rustfmt`, `clippy`)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Node.js**: Version $\ge 20.x$ (LTS recommended)
- **pnpm**: Version $\ge 9.x$
  ```bash
  npm install -g pnpm
  ```
- **Tauri Prerequisites** (if building desktop application):
  - Linux: `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget`, `file`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`
  - Windows: Visual Studio C++ Build Tools & WebView2
  - macOS: Xcode Command Line Tools

### 1.2 Database & In-Memory Store
- **PostgreSQL** $\ge 15$ or **SQLite** (for local testing)
- **DragonflyDB** or **Redis** $\ge 7.0$

### 1.3 System Network Binaries
Ensure standard diagnostic binaries are available in your system path:
```bash
# Ubuntu / Debian
sudo apt-get install -y iputils-ping traceroute mtr iperf3 tcpdump dnsutils

# Arch Linux
sudo pacman -S iputils traceroute mtr iperf3 tcpdump bind-tools

# macOS (Homebrew)
brew install iputils mtr iperf3
```

---

## 2. Repository Structure

```text
r-speedtest/
├── md/                       # Project Documentation & Architecture Specs
│   ├── prompt.md             # Original Specification Prompt
│   ├── PRD.md                # Product Requirements Document
│   ├── TDD.md                # Technical Design Document
│   ├── ADR.md                # Architecture Decision Records
│   ├── BRAINSTORM.md         # Brainstorming & Edge Cases
│   ├── DEPLOYMENT.md         # Production & Docker Deployment
│   ├── SETUP.md              # Local Setup Guide (This file)
│   └── PHASES.md             # Phased Implementation Roadmap
├── qiluo_admin/              # Rust Backend (Axum + Sea-ORM + Workers)
├── qiluo_vue/                # Vue 3 Frontend (Vite + TS + UnoCSS)
└── qiluo_tauri/              # Tauri Desktop Application Wrapper
```

---

## 3. Quick Start (Using Docker for Dependencies)

The fastest way to spin up the local database and DragonflyDB is using Docker:

```bash
# 1. Start DragonflyDB and PostgreSQL in background
docker run -d --name local-dragonfly -p 6379:6379 docker.dragonflydb.io/dragonflydb/dragonfly:latest
docker run -d --name local-postgres -p 5432:5432 -e POSTGRES_USER=qiluo -e POSTGRES_PASSWORD=qiluo_password -e POSTGRES_DB=qiluo_dev postgres:16-alpine
```

---

## 4. Backend Setup (`qiluo_admin`)

### 4.1 Configure Application Settings
Edit or verify `qiluo_admin/config/development.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 3000

database:
  db_type: "postgres"
  url: "postgres://qiluo:qiluo_password@127.0.0.1:5432/qiluo_dev"
  max_connections: 20
  min_connections: 5

cache:
  cache_type: "redis"
  url: "redis://127.0.0.1:6379"
  namespace: "rspeedtest"

workers:
  num_workers: 8
  queues:
    - "default"
    - "net_tool"
    - "scenario"
    - "mailer"
```

### 4.2 Initialize Database Schema & Migrations
Run the initial SQL script or execute Sea-ORM migrations:

```bash
cd qiluo_admin

# Method A: Direct SQL Import (PostgreSQL)
PGPASSWORD=qiluo_password psql -h 127.0.0.1 -U qiluo -d qiluo_dev -f qiluo_pg.sql

# Method B: Sea-ORM Migration CLI
cargo run --manifest-path migration/Cargo.toml
```

### 4.3 Run the Backend Server
```bash
cd qiluo_admin
cargo run
```
The Axum backend will start on `http://127.0.0.1:3000`.

---

## 5. Frontend Setup (`qiluo_vue`)

### 5.1 Install Dependencies
```bash
cd qiluo_vue
pnpm install
```

### 5.2 Configure Environment Variables
Verify `.env.dev`:
```ini
# Backend API Base URL
VITE_API_BASE_PATH=http://127.0.0.1:3000
VITE_BASE_PATH=/
VITE_APP_TITLE=Network Swiss Army & Speedtest
```

### 5.3 Start Development Server
```bash
pnpm dev
```
The Vue 3 UI will launch at `http://127.0.0.1:4000` (or `http://localhost:5173`).

---

## 6. Desktop Application Setup (`qiluo_tauri`)

To run the platform as a cross-platform native desktop application:

```bash
cd qiluo_tauri
pnpm install
pnpm tauri dev
```

---

## 7. Troubleshooting & Common Pitfalls

| Issue | Cause | Solution |
| :--- | :--- | :--- |
| `ping: socket: Operation not permitted` | ICMP raw socket permissions restricted on Linux | Run `sudo setcap cap_net_raw+ep $(which ping)` or run in Docker container with `--cap-add=NET_RAW`. |
| `Failed to initialize cache manager` | DragonflyDB / Redis is not running or port 6379 blocked | Verify `docker ps` and test with `redis-cli ping`. |
| `Database connection refused` | PostgreSQL container is still starting or password mismatch | Check PostgreSQL logs with `docker logs local-postgres`. |
| `pcap file permission denied` | Worker process does not have write access to `/var/log/r-speedtest/pcaps/` | Create directory with `sudo mkdir -p /var/log/r-speedtest/pcaps && sudo chown -R $USER /var/log/r-speedtest`. |
| `Vite proxy ECONNREFUSED` | Backend `qiluo_admin` is not running on port 3000 | Ensure `cargo run` in `qiluo_admin` has compiled and is listening on port 3000. |
