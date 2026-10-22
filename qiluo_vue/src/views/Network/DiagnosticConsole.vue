<script setup lang="ts">
import { ref } from 'vue'

const activeTool = ref('ping')
const targetHost = ref('1.1.1.1')
const packetCount = ref(4)
const packetSize = ref(64)
const isRunning = ref(false)
const terminalLogs = ref<string[]>([
  '[System] Network Swiss Army Diagnostic Console initialized.',
  '[System] Ready for execution.'
])

const runDiagnostic = () => {
  isRunning.value = true
  terminalLogs.value.push(`[Job Triggered] Running ${activeTool.value.toUpperCase()} against ${targetHost.value}...`)
  setTimeout(() => {
    terminalLogs.value.push(`[Job Result] 4 packets transmitted, 4 received, 0% packet loss, time 3004ms`)
    terminalLogs.value.push(`[Job Result] rtt min/avg/max/mdev = 12.341/14.512/18.231/1.942 ms`)
    isRunning.value = false
  }, 1200)
}
</script>

<template>
  <div class="p-4 space-y-4">
    <div class="bg-white dark:bg-[#1f1f23] p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-800">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100">Network Diagnostic Console</h2>
          <p class="text-sm text-gray-500">Execute low-level network connectivity tests with real-time telemetry.</p>
        </div>
        <div class="flex items-center gap-2">
          <button
            v-for="tool in ['ping', 'traceroute', 'mtr', 'dns', 'portcheck']"
            :key="tool"
            @click="activeTool = tool"
            :class="[
              'px-3 py-1.5 rounded-lg text-xs font-semibold uppercase tracking-wider transition-all',
              activeTool === tool
                ? 'bg-blue-600 text-white shadow-md shadow-blue-500/20'
                : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200'
            ]"
          >
            {{ tool }}
          </button>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
        <div class="md:col-span-2">
          <label class="block text-xs font-medium text-gray-500 mb-1">Target Host / IP</label>
          <input
            v-model="targetHost"
            type="text"
            placeholder="e.g. 8.8.8.8 or example.com"
            class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label class="block text-xs font-medium text-gray-500 mb-1">Packet Count</label>
          <input
            v-model="packetCount"
            type="number"
            min="1"
            max="1000"
            class="w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div class="flex items-end">
          <button
            @click="runDiagnostic"
            :disabled="isRunning"
            class="w-full py-2 px-4 rounded-lg bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white text-sm font-semibold flex items-center justify-center gap-2 shadow-lg shadow-blue-500/20 transition-all"
          >
            <span v-if="isRunning" class="animate-spin">🌀</span>
            <span>{{ isRunning ? 'Executing...' : 'Run Diagnostic' }}</span>
          </button>
        </div>
      </div>

      <!-- Live Terminal Output -->
      <div class="bg-gray-950 text-emerald-400 font-mono text-xs p-4 rounded-xl shadow-inner min-h-[220px] max-h-[360px] overflow-y-auto space-y-1">
        <div v-for="(log, idx) in terminalLogs" :key="idx" class="leading-relaxed">
          {{ log }}
        </div>
      </div>
    </div>
  </div>
</template>
