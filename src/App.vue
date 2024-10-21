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
  <main class="p-12 flex flex-col gap-12">
    <div class="grid grid-cols-6 grid-rows-6 gap-0.5 p-1 bg-stone-950 rounded-xl w-2/3">
      <Tile v-for="(connections, index) in data" :key="index" :connections="connections" />
    </div>
    <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ data }}</p>
  </main>
</template>
