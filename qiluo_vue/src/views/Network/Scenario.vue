<script setup lang="ts">
import { ref } from 'vue'

const scenarios = ref([
  {
    id: 'sc-01',
    name: 'Customer ONT Quality Healthcheck',
    steps: ['Ping Gateway', 'DNS Resolution', 'TR-143 Download', 'UDP Jitter'],
    lastRun: '10 mins ago',
    healthScore: 98
  },
  {
    id: 'sc-02',
    name: 'Core POP Backbone Verification',
    steps: ['MTR Hop Analysis', 'iPerf3 10G TCP', 'UDP Echo 5000 pkts'],
    lastRun: '1 hour ago',
    healthScore: 100
  }
])
</script>

<template>
  <div class="p-4 space-y-4">
    <div class="bg-white dark:bg-[#1f1f23] p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-800">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100">Diagnostic Scenario Automation</h2>
          <p class="text-sm text-gray-500">Chain multiple network diagnostics into automated, conditional DAG workflows.</p>
        </div>
        <button class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white font-semibold text-xs uppercase tracking-wider shadow-md shadow-blue-500/20">
          + Create Scenario
        </button>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div
          v-for="sc in scenarios"
          :key="sc.id"
          class="p-5 rounded-xl border border-gray-100 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-900/50 hover:border-blue-500/40 transition-all"
        >
          <div class="flex items-center justify-between">
            <h3 class="font-bold text-gray-800 dark:text-gray-100">{{ sc.name }}</h3>
            <span class="px-2.5 py-1 rounded-full text-xs font-bold bg-emerald-500/10 text-emerald-500">
              Score: {{ sc.healthScore }}%
            </span>
          </div>

          <div class="flex items-center gap-2 mt-3 flex-wrap">
            <span
              v-for="(step, sIdx) in sc.steps"
              :key="sIdx"
              class="px-2 py-0.5 rounded bg-gray-200 dark:bg-gray-800 text-gray-700 dark:text-gray-300 text-[11px] font-medium"
            >
              {{ sIdx + 1 }}. {{ step }}
            </span>
          </div>

          <div class="flex items-center justify-between mt-6 pt-4 border-t border-gray-100 dark:border-gray-800">
            <span class="text-xs text-gray-400">Last run: {{ sc.lastRun }}</span>
            <button class="px-3 py-1.5 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-xs font-semibold">
              Run Workflow
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
