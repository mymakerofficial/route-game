<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Tile from "@/components/Tile.vue";
import {Connections} from "@/types.ts";
import {useQuery} from "@tanstack/vue-query";

const { data } = useQuery({
  queryKey: ['get_possible_connections'],
  queryFn: async () => {
    return await invoke<Connections>("get_possible_connections");
  },
  initialData: []
})
</script>

<template>
  <main class="container">
    <p>{{ data }}</p>
    <div class="tile-container">
      <Tile v-for="(connections, index) in data" :key="index" :connections="connections" />
    </div>
  </main>
</template>

<style>
.tile-container {
  display: grid;
  grid-template-columns: repeat(6, minmax(0, 1fr));
  grid-template-rows: repeat(6, minmax(0, 1fr));
  max-height: 70vh;
  aspect-ratio: 1;
  gap: 0.1rem;
  padding: 0.1rem;
  background: #311202;
}
</style>