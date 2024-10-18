<script setup lang="ts">
import {computed, ref} from "vue";
import { invoke } from "@tauri-apps/api/core";

const connections = ref([ [ 0, 1 ], [ 2, 5 ], [ 3, 7 ], [ 4, 6 ] ]);

const scale = 33.33;

function getCoords(i: number): { x: number, y: number} {
  switch (i) {
    case 0: return { x: 1, y: 3 };
    case 1: return { x: 2, y: 3 };
    case 2: return { x: 3, y: 2 };
    case 3: return { x: 3, y: 1 };
    case 4: return { x: 2, y: 0 };
    case 5: return { x: 1, y: 0 };
    case 6: return { x: 0, y: 1 };
    case 7: return { x: 0, y: 2 };
  }
}

const translatedConnections = computed(() => {
  return connections.value.map((connection) => {
    const lower = getCoords(connection[0]);
    const higher = getCoords(connection[1]);
    return {lower, higher};
  });
})

async function submit() {
  connections.value = await invoke("test");
}
</script>

<template>
  <main class="container">
    <button @click="submit">submit</button>
    <p>{{ connections }}</p>
    <svg width="100" height="100" style="background: #431407">
      <template
        v-for="(connection, index) in translatedConnections"
        :key="index"
      >
        <path
          :d="`M ${connection.lower.x * scale} ${connection.lower.y * scale} Q 50 50 ${connection.higher.x * scale} ${connection.higher.y * scale}`"
          stroke="#431407"
          stroke-width="10"
          fill="transparent"
        />
        <path
          :d="`M ${connection.lower.x * scale} ${connection.lower.y * scale} Q 50 50 ${connection.higher.x * scale} ${connection.higher.y * scale}`"
          stroke="#ca8a04"
          stroke-width="5"
          fill="transparent"
        />
        <path
          :d="`M ${connection.lower.x * scale} ${connection.lower.y * scale} Q 50 50 ${connection.higher.x * scale} ${connection.higher.y * scale}`"
          stroke="#fed7aa"
          stroke-width="3"
          fill="transparent"
        />
      </template>
    </svg>
  </main>
</template>
