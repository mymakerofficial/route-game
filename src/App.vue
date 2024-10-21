<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {Tile} from "@/types.ts";
import {useQuery} from "@tanstack/vue-query";
import GameGrid from "@/components/GameGrid.vue";
import TileStack from "@/components/TileStack.vue";

const { data } = useQuery({
  queryKey: ['get_tiles'],
  queryFn: async (): Promise<Tile[]> => {
    return await invoke("get_tile_stack");
  },
  initialData: []
})
</script>

<template>
  <main class="w-screen overflow-x-hidden">
    <div class="h-screen p-12">
      <GameGrid :tiles="data" class="max-h-full" />
    </div>
    <div class="p-12 flex flex-col gap-12">
      <TileStack :tiles="data" />
      <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ data }}</p>
    </div>
  </main>
</template>
