<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {Tile} from "@/types.ts";
import {useQuery} from "@tanstack/vue-query";
import GameGrid from "@/components/GameGrid.vue";

const { data } = useQuery({
  queryKey: ['get_tiles'],
  queryFn: async (): Promise<Tile[]> => {
    return await invoke("get_tile_stack");
  },
  initialData: []
})
</script>

<template>
  <main class="flex flex-col">
    <div class="h-screen p-12">
      <GameGrid :tiles="data" />
    </div>
    <div class="p-12">
      <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ data }}</p>
    </div>
  </main>
</template>
