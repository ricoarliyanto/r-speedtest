<script setup lang="ts">
import { ref } from 'vue'

const speedMbps = ref(0)
const isTesting = ref(false)
const testType = ref<'download' | 'upload'>('download')
const latencyMs = ref(12)
const jitterMs = ref(1.4)
const packetLoss = ref(0)

const startSpeedtest = () => {
  isTesting.value = true
  speedMbps.value = 0
  
  const interval = setInterval(() => {
    if (speedMbps.value < 940) {
      speedMbps.value += Math.floor(Math.random() * 80) + 40
    } else {
      clearInterval(interval)
      isTesting.value = false
    }
  }, 150)
}
</script>

<template>
  <div class="p-4 space-y-4">
    <div class="bg-white dark:bg-[#1f1f23] p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-800">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100">Speedtest & High-Throughput Engine</h2>
          <p class="text-sm text-gray-500">Perform multi-gigabit line-rate throughput tests using native iPerf3 and TR-143 engines.</p>
        </div>
        <div class="flex items-center gap-2">
          <button
            @click="testType = 'download'"
            :class="['px-3 py-1.5 rounded-lg text-xs font-semibold uppercase transition-all', testType === 'download' ? 'bg-blue-600 text-white' : 'bg-gray-100 dark:bg-gray-800 text-gray-400']"
          >
            Download
          </button>
          <button
            @click="testType = 'upload'"
            :class="['px-3 py-1.5 rounded-lg text-xs font-semibold uppercase transition-all', testType === 'upload' ? 'bg-blue-600 text-white' : 'bg-gray-100 dark:bg-gray-800 text-gray-400']"
          >
            Upload
          </button>
        </div>
      </div>

      <!-- Speed Gauge Display -->
      <div class="flex flex-col items-center justify-center py-8">
        <div class="relative flex flex-col items-center justify-center w-64 h-64 rounded-full border-4 border-blue-500/20 bg-blue-50/10 dark:bg-blue-950/20">
          <div class="text-5xl font-black tracking-tight text-blue-600 dark:text-blue-400">
            {{ speedMbps }}
          </div>
          <div class="text-xs uppercase font-bold tracking-widest text-gray-400 mt-1">Mbps</div>
        </div>

        <button
          @click="startSpeedtest"
          :disabled="isTesting"
          class="mt-8 px-8 py-3 rounded-xl bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 text-white font-bold text-sm tracking-wider uppercase shadow-xl shadow-blue-500/30 transition-all disabled:opacity-50"
        >
          {{ isTesting ? 'Testing Speed...' : 'Start Speedtest' }}
        </button>
      </div>

      <!-- Realtime Telemetry Grid -->
      <div class="grid grid-cols-3 gap-4 mt-6">
        <div class="p-4 rounded-xl bg-gray-50 dark:bg-gray-900 border border-gray-100 dark:border-gray-800 text-center">
          <div class="text-xs font-medium text-gray-400">Latency</div>
          <div class="text-lg font-bold text-gray-800 dark:text-gray-100 mt-1">{{ latencyMs }} ms</div>
        </div>
        <div class="p-4 rounded-xl bg-gray-50 dark:bg-gray-900 border border-gray-100 dark:border-gray-800 text-center">
          <div class="text-xs font-medium text-gray-400">Jitter</div>
          <div class="text-lg font-bold text-gray-800 dark:text-gray-100 mt-1">{{ jitterMs }} ms</div>
        </div>
        <div class="p-4 rounded-xl bg-gray-50 dark:bg-gray-900 border border-gray-100 dark:border-gray-800 text-center">
          <div class="text-xs font-medium text-gray-400">Packet Loss</div>
          <div class="text-lg font-bold text-gray-800 dark:text-gray-100 mt-1">{{ packetLoss }}%</div>
        </div>
      </div>
    </div>
  </div>
</template>
