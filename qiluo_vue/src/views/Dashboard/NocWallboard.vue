<template>
  <div class="noc-container">
    <!-- Header Banner -->
    <header class="noc-header">
      <div class="header-left">
        <div class="live-pulse"></div>
        <h1>24/7 GLOBAL NOC OPERATIONS CENTER</h1>
        <span class="badge-live">LIVE TELEMETRY</span>
      </div>
      <div class="header-right">
        <div class="clock-display">{{ currentTime }}</div>
        <div class="status-indicator">
          <span class="status-dot online"></span>
          <span>NOC ENGINE RUNNING</span>
        </div>
      </div>
    </header>

    <!-- Top KPI Grid -->
    <div class="kpi-grid">
      <div class="kpi-card glow-cyan">
        <div class="kpi-title">TOTAL AGGREGATED BANDWIDTH</div>
        <div class="kpi-value">{{ totalBandwidth }} <span class="unit">Gbps</span></div>
        <div class="kpi-sub">Peak: 9.84 Gbps | TR-143 Verified</div>
      </div>

      <div class="kpi-card glow-green">
        <div class="kpi-title">FLEET HEALTH SCORE</div>
        <div class="kpi-value text-green">{{ healthScore }}<span class="unit">/100</span></div>
        <div class="kpi-sub">Composite Score across all Edge Nodes</div>
      </div>

      <div class="kpi-card glow-purple">
        <div class="kpi-title">ACTIVE DIAGNOSTIC JOBS</div>
        <div class="kpi-value text-purple">{{ activeJobsCount }}</div>
        <div class="kpi-sub">{{ completedJobsCount }} executed today (99.8% Success)</div>
      </div>

      <div class="kpi-card glow-amber">
        <div class="kpi-title">NETWORK AVG JITTER / LOSS</div>
        <div class="kpi-value text-amber">{{ avgJitter }} <span class="unit">ms</span> / {{ avgLoss }}<span class="unit">%</span></div>
        <div class="kpi-sub">RFC 3550 Compliant Delivery</div>
      </div>
    </div>

    <!-- Main Dashboard Row -->
    <div class="main-grid">
      <!-- Edge Probes Grid -->
      <div class="panel node-fleet-panel">
        <div class="panel-header">
          <h3><i class="el-icon-cpu"></i> DISTRIBUTED EDGE PROBE FLEET</h3>
          <span class="node-count">{{ onlineNodeCount }}/{{ nodes.length }} PROBES ONLINE</span>
        </div>
        <div class="node-grid">
          <div v-for="node in nodes" :key="node.id" class="node-card" :class="node.status">
            <div class="node-header">
              <span class="node-name">{{ node.name }}</span>
              <span class="node-status-badge" :class="node.status">{{ node.status.toUpperCase() }}</span>
            </div>
            <div class="node-meta">
              <div><strong>IP:</strong> {{ node.ip_address }}</div>
              <div><strong>Region:</strong> {{ node.region }}</div>
              <div><strong>Last Beat:</strong> {{ formatTime(node.last_heartbeat) }}</div>
            </div>
            <div class="node-caps">
              <span v-for="cap in node.capabilities" :key="cap" class="cap-tag">{{ cap }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Live Telemetry Stream -->
      <div class="panel telemetry-panel">
        <div class="panel-header">
          <h3><i class="el-icon-video-play"></i> LIVE DIAGNOSTIC STREAM</h3>
          <span class="stream-badge">WEBSOCKET CONNECTED</span>
        </div>
        <div class="log-terminal">
          <div v-for="(log, idx) in telemetryLogs" :key="idx" class="terminal-line">
            <span class="log-time">[{{ log.time }}]</span>
            <span class="log-tag" :class="log.level">[{{ log.level.toUpperCase() }}]</span>
            <span class="log-msg">{{ log.message }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const currentTime = ref('');
const totalBandwidth = ref('8.42');
const healthScore = ref(98);
const activeJobsCount = ref(4);
const completedJobsCount = ref(1284);
const avgJitter = ref('0.42');
const avgLoss = ref('0.02');
const onlineNodeCount = ref(4);

const nodes = ref([
  { id: 'node_1', name: 'Edge-Jakarta-IDC3', ip_address: '103.28.14.10', region: 'ID-JKT', status: 'online', last_heartbeat: new Date().toISOString(), capabilities: ['ping', 'traceroute', 'iperf3', 'tr143'] },
  { id: 'node_2', name: 'Edge-Singapore-Equinix', ip_address: '139.180.200.12', region: 'SG-SIN', status: 'online', last_heartbeat: new Date().toISOString(), capabilities: ['ping', 'dns', 'iperf3', 'tr143', 'tcpdump'] },
  { id: 'node_3', name: 'Edge-Surabaya-Cyber', ip_address: '103.111.20.5', region: 'ID-SUB', status: 'online', last_heartbeat: new Date().toISOString(), capabilities: ['ping', 'portcheck', 'iperf3'] },
  { id: 'node_4', name: 'Edge-Tokyo-TYO2', ip_address: '108.61.200.8', region: 'JP-TYO', status: 'online', last_heartbeat: new Date().toISOString(), capabilities: ['ping', 'traceroute', 'tr143'] },
]);

const telemetryLogs = ref([
  { time: '06:45:10', level: 'info', message: 'Edge-Jakarta-IDC3 completed TR-143 Download Test: 942.50 Mbps (ROM: 12.4ms)' },
  { time: '06:45:12', level: 'info', message: 'Scenario sc_full_isp executed: Health Score 100/100 across 5 steps' },
  { time: '06:45:18', level: 'info', message: 'UDP Echo high-precision probe: 50 packets sent, 0% loss, Jitter 0.38ms' },
  { time: '06:45:25', level: 'warn', message: 'BBR Congestion Window adjusted on Tokyo Probe TYO2' },
  { time: '06:45:30', level: 'info', message: 'DragonflyDB queue:net_tool dispatched job_iperf3_1724309130' }
]);

let timer: any = null;

const updateClock = () => {
  const now = new Date();
  currentTime.value = now.toUTCString();
};

const formatTime = (ts: string) => {
  return new Date(ts).toLocaleTimeString();
};

onMounted(() => {
  updateClock();
  timer = setInterval(updateClock, 1000);
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
});
</script>

<style scoped>
.noc-container {
  min-height: 100vh;
  background-color: #060913;
  color: #e2e8f0;
  padding: 24px;
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
  box-sizing: border-box;
}

.noc-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #1e293b;
  padding-bottom: 16px;
  margin-bottom: 24px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-left h1 {
  font-size: 20px;
  font-weight: 800;
  letter-spacing: 1px;
  margin: 0;
  color: #38bdf8;
}

.live-pulse {
  width: 12px;
  height: 12px;
  background-color: #10b981;
  border-radius: 50%;
  box-shadow: 0 0 12px #10b981;
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% { transform: scale(0.95); opacity: 0.8; }
  50% { transform: scale(1.3); opacity: 1; }
  100% { transform: scale(0.95); opacity: 0.8; }
}

.badge-live {
  background: rgba(56, 189, 248, 0.15);
  color: #38bdf8;
  border: 1px solid #38bdf8;
  font-size: 11px;
  font-weight: 700;
  padding: 2px 8px;
  border-radius: 4px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.clock-display {
  font-family: 'Courier New', monospace;
  color: #94a3b8;
  font-size: 14px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  font-weight: 600;
  color: #10b981;
}

.status-dot.online {
  width: 8px;
  height: 8px;
  background: #10b981;
  border-radius: 50%;
}

.kpi-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.kpi-card {
  background: #0f172a;
  border: 1px solid #1e293b;
  border-radius: 8px;
  padding: 16px;
  transition: all 0.2s ease;
}

.kpi-card.glow-cyan { border-top: 3px solid #06b6d4; }
.kpi-card.glow-green { border-top: 3px solid #10b981; }
.kpi-card.glow-purple { border-top: 3px solid #a855f7; }
.kpi-card.glow-amber { border-top: 3px solid #f59e0b; }

.kpi-title {
  font-size: 11px;
  font-weight: 700;
  color: #64748b;
  letter-spacing: 0.5px;
  margin-bottom: 8px;
}

.kpi-value {
  font-size: 32px;
  font-weight: 900;
  color: #f8fafc;
}

.kpi-value .unit {
  font-size: 16px;
  font-weight: 500;
  color: #94a3b8;
}

.text-green { color: #10b981; }
.text-purple { color: #c084fc; }
.text-amber { color: #fbbf24; }

.kpi-sub {
  font-size: 12px;
  color: #64748b;
  margin-top: 6px;
}

.main-grid {
  display: grid;
  grid-template-columns: 3fr 2fr;
  gap: 16px;
}

.panel {
  background: #0f172a;
  border: 1px solid #1e293b;
  border-radius: 8px;
  padding: 16px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #1e293b;
  padding-bottom: 12px;
  margin-bottom: 16px;
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 700;
  color: #e2e8f0;
}

.node-count, .stream-badge {
  font-size: 11px;
  color: #38bdf8;
  font-weight: 600;
}

.node-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.node-card {
  background: #1e293b;
  border-radius: 6px;
  padding: 12px;
  border: 1px solid #334155;
}

.node-card.online { border-left: 4px solid #10b981; }
.node-card.offline { border-left: 4px solid #ef4444; }

.node-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.node-name {
  font-weight: 700;
  font-size: 13px;
  color: #f8fafc;
}

.node-status-badge {
  font-size: 10px;
  font-weight: 800;
  padding: 2px 6px;
  border-radius: 3px;
}

.node-status-badge.online { background: rgba(16, 185, 129, 0.2); color: #10b981; }
.node-status-badge.offline { background: rgba(239, 68, 68, 0.2); color: #ef4444; }

.node-meta {
  font-size: 11px;
  color: #94a3b8;
  display: flex;
  flex-direction: column;
  gap: 3px;
  margin-bottom: 8px;
}

.node-caps {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.cap-tag {
  background: #0f172a;
  border: 1px solid #334155;
  color: #38bdf8;
  font-size: 10px;
  padding: 1px 5px;
  border-radius: 3px;
}

.log-terminal {
  background: #020617;
  border: 1px solid #1e293b;
  border-radius: 6px;
  padding: 12px;
  height: 380px;
  overflow-y: auto;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.terminal-line {
  display: flex;
  gap: 8px;
}

.log-time { color: #64748b; }
.log-tag.info { color: #38bdf8; }
.log-tag.warn { color: #fbbf24; }
.log-tag.critical { color: #ef4444; }
.log-msg { color: #e2e8f0; }
</style>
