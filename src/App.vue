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
  <main class="flex flex-col">
    <div class="h-screen p-12">
      <div class="grid grid-cols-6 grid-rows-6 gap-0.5 p-1 bg-stone-950 rounded-xl h-full aspect-square">
        <Tile v-for="(connections, index) in data" :key="index" :connections="connections" />
      </div>
    </div>
    <div class="p-12">
      <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ data }}</p>
    </div>
  </main>
</template>
