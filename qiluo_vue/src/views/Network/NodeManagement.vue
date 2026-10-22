<script setup lang="ts">
import { ref } from 'vue'

const nodes = ref([
  {
    id: 'node-jkt',
    name: 'Jakarta Datacenter Master',
    ip: '103.20.14.1',
    region: 'Jakarta, ID',
    status: 'online',
    cpu: '18%',
    ram: '34%'
  },
  {
    id: 'node-bdg',
    name: 'Bandung POP Edge Probe',
    ip: '103.20.15.10',
    region: 'Bandung, ID',
    status: 'online',
    cpu: '8%',
    ram: '22%'
  },
  {
    id: 'node-pnk',
    name: 'Pontianak Substation Probe',
    ip: '103.20.16.5',
    region: 'Pontianak, ID',
    status: 'online',
    cpu: '12%',
    ram: '28%'
  }
])
</script>

<template>
  <div class="p-4 space-y-4">
    <div class="bg-white dark:bg-[#1f1f23] p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-800">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100">Edge Probe & Multi-Node Topology</h2>
          <p class="text-sm text-gray-500">Monitor distributed diagnostic probe servers deployed across regional POPs.</p>
        </div>
        <button class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white font-semibold text-xs uppercase tracking-wider shadow-md shadow-blue-500/20">
          + Add Probe Node
        </button>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div
          v-for="node in nodes"
          :key="node.id"
          class="p-5 rounded-xl border border-gray-100 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-900/50"
        >
          <div class="flex items-center justify-between">
            <span class="text-xs font-mono text-gray-400">{{ node.ip }}</span>
            <span class="flex items-center gap-1.5 text-xs font-semibold text-emerald-500">
              <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
              Online
            </span>
          </div>

          <h3 class="font-bold text-base text-gray-800 dark:text-gray-100 mt-2">{{ node.name }}</h3>
          <p class="text-xs text-gray-500 mt-0.5">{{ node.region }}</p>

          <div class="grid grid-cols-2 gap-2 mt-4 pt-4 border-t border-gray-100 dark:border-gray-800">
            <div class="text-xs">
              <span class="text-gray-400">CPU Load:</span>
              <span class="font-bold text-gray-700 dark:text-gray-200 ml-1">{{ node.cpu }}</span>
            </div>
            <div class="text-xs">
              <span class="text-gray-400">RAM:</span>
              <span class="font-bold text-gray-700 dark:text-gray-200 ml-1">{{ node.ram }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
