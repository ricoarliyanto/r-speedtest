<script setup lang="ts">
import { ref } from 'vue'

const jobs = ref([
  {
    id: 'job-9812',
    tool: 'PING',
    target: '8.8.8.8',
    status: 'COMPLETED',
    duration: '2.4s',
    user: 'admin',
    time: '2 mins ago'
  },
  {
    id: 'job-9811',
    tool: 'IPERF3',
    target: '10.20.0.15',
    status: 'COMPLETED',
    duration: '30.1s',
    user: 'rico',
    time: '15 mins ago'
  },
  {
    id: 'job-9810',
    tool: 'TR143_DL',
    target: 'speed.isp.net',
    status: 'COMPLETED',
    duration: '10.0s',
    user: 'rico',
    time: '1 hour ago'
  }
])
</script>

<template>
  <div class="p-4 space-y-4">
    <div class="bg-white dark:bg-[#1f1f23] p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-800">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100">Job Execution History & Audit Logs</h2>
          <p class="text-sm text-gray-500">Search, inspect and audit diagnostic and speedtest execution records.</p>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs">
          <thead>
            <tr class="border-b border-gray-100 dark:border-gray-800 text-gray-400 uppercase tracking-wider">
              <th class="py-3 px-4">Job ID</th>
              <th class="py-3 px-4">Tool</th>
              <th class="py-3 px-4">Target Host</th>
              <th class="py-3 px-4">Status</th>
              <th class="py-3 px-4">Duration</th>
              <th class="py-3 px-4">Requested By</th>
              <th class="py-3 px-4">Time</th>
              <th class="py-3 px-4 text-right">Action</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-100 dark:divide-gray-800 text-gray-700 dark:text-gray-300">
            <tr v-for="job in jobs" :key="job.id" class="hover:bg-gray-50 dark:hover:bg-gray-900/50">
              <td class="py-3 px-4 font-mono font-bold text-blue-600 dark:text-blue-400">{{ job.id }}</td>
              <td class="py-3 px-4">
                <span class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 font-bold">{{ job.tool }}</span>
              </td>
              <td class="py-3 px-4 font-mono">{{ job.target }}</td>
              <td class="py-3 px-4">
                <span class="px-2 py-0.5 rounded text-[11px] font-bold bg-emerald-500/10 text-emerald-500">
                  {{ job.status }}
                </span>
              </td>
              <td class="py-3 px-4">{{ job.duration }}</td>
              <td class="py-3 px-4">{{ job.user }}</td>
              <td class="py-3 px-4 text-gray-400">{{ job.time }}</td>
              <td class="py-3 px-4 text-right">
                <button class="text-blue-600 hover:underline font-semibold">View Result</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
