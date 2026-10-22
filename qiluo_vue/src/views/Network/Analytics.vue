<template>
  <div class="analytics-container">
    <div class="header-section">
      <h2><i class="el-icon-data-analysis"></i> Historical Network Telemetry & SLA Delivery</h2>
      <div class="filter-controls">
        <select class="time-filter">
          <option value="24h">Last 24 Hours</option>
          <option value="7d">Last 7 Days</option>
          <option value="30d">Last 30 Days</option>
        </select>
      </div>
    </div>

    <!-- Analytics KPI Cards -->
    <div class="analytics-kpi-grid">
      <div class="analytics-card">
        <div class="label">SLA Compliance Rate</div>
        <div class="val text-success">99.94%</div>
        <div class="sub">Target: >= 99.5% Delivery</div>
      </div>
      <div class="analytics-card">
        <div class="label">Average Round-Trip Latency</div>
        <div class="val text-primary">14.8 ms</div>
        <div class="sub">Across 4 Edge Probe Zones</div>
      </div>
      <div class="analytics-card">
        <div class="label">P99 Latency (Tail Delay)</div>
        <div class="val text-warning">28.4 ms</div>
        <div class="sub">Under Heavy Load (iPerf3 10G)</div>
      </div>
      <div class="analytics-card">
        <div class="label">Total Diagnostics Run</div>
        <div class="val">18,429</div>
        <div class="sub">Zero Arbitrary Shell Violations</div>
      </div>
    </div>

    <!-- Charts Area -->
    <div class="charts-grid">
      <div class="chart-panel">
        <h3>ISP Throughput vs SLA Baseline (Mbps)</h3>
        <div class="chart-mock">
          <div class="bar-group" v-for="h in [820, 940, 910, 960, 890, 950, 990, 930, 940, 980, 970, 990]" :key="h">
            <div class="bar" :style="{ height: `${(h / 1000) * 180}px` }"></div>
            <span class="bar-label">{{ h }}M</span>
          </div>
        </div>
      </div>

      <div class="chart-panel">
        <h3>Latency Distribution by Region (ms)</h3>
        <div class="latency-table">
          <div class="table-row header">
            <span>Region</span>
            <span>Avg RTT</span>
            <span>Jitter (RFC 3550)</span>
            <span>Packet Loss</span>
          </div>
          <div class="table-row">
            <span>ID-JKT (Jakarta IDC3)</span>
            <span class="val">4.2 ms</span>
            <span>0.18 ms</span>
            <span class="text-success">0.00%</span>
          </div>
          <div class="table-row">
            <span>SG-SIN (Singapore Equinix)</span>
            <span class="val">12.8 ms</span>
            <span>0.42 ms</span>
            <span class="text-success">0.01%</span>
          </div>
          <div class="table-row">
            <span>ID-SUB (Surabaya Cyber)</span>
            <span class="val">18.4 ms</span>
            <span>0.35 ms</span>
            <span class="text-success">0.00%</span>
          </div>
          <div class="table-row">
            <span>JP-TYO (Tokyo TYO2)</span>
            <span class="val">68.5 ms</span>
            <span>0.82 ms</span>
            <span class="text-success">0.02%</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Analytics View Component
</script>

<style scoped>
.analytics-container {
  padding: 24px;
  background: #0b0f19;
  min-height: 100vh;
  color: #f1f5f9;
  font-family: 'Inter', system-ui, sans-serif;
  box-sizing: border-box;
}

.header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  border-bottom: 1px solid #1e293b;
  padding-bottom: 16px;
}

.header-section h2 {
  margin: 0;
  font-size: 18px;
  color: #38bdf8;
}

.time-filter {
  background: #1e293b;
  color: #f8fafc;
  border: 1px solid #334155;
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 13px;
}

.analytics-kpi-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.analytics-card {
  background: #0f172a;
  border: 1px solid #1e293b;
  border-radius: 8px;
  padding: 16px;
}

.analytics-card .label {
  font-size: 12px;
  color: #94a3b8;
  margin-bottom: 8px;
}

.analytics-card .val {
  font-size: 28px;
  font-weight: 800;
  color: #f8fafc;
}

.text-success { color: #10b981; }
.text-primary { color: #38bdf8; }
.text-warning { color: #f59e0b; }

.analytics-card .sub {
  font-size: 11px;
  color: #64748b;
  margin-top: 4px;
}

.charts-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.chart-panel {
  background: #0f172a;
  border: 1px solid #1e293b;
  border-radius: 8px;
  padding: 20px;
}

.chart-panel h3 {
  margin: 0 0 20px 0;
  font-size: 14px;
  color: #e2e8f0;
}

.chart-mock {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  height: 220px;
  border-bottom: 2px solid #334155;
  padding-bottom: 8px;
}

.bar-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  flex: 1;
}

.bar {
  width: 24px;
  background: linear-gradient(180deg, #38bdf8 0%, #0284c7 100%);
  border-radius: 4px 4px 0 0;
}

.bar-label {
  font-size: 10px;
  color: #94a3b8;
}

.latency-table {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.table-row {
  display: grid;
  grid-template-columns: 2fr 1fr 1.2fr 1fr;
  padding: 12px;
  background: #1e293b;
  border-radius: 6px;
  font-size: 12px;
  align-items: center;
}

.table-row.header {
  background: #0b0f19;
  color: #94a3b8;
  font-weight: 700;
}
</style>
