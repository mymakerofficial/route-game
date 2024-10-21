<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {Tile} from "@/types.ts";
import {useQuery} from "@tanstack/vue-query";
import GameGrid from "@/components/GameGrid.vue";
import TileStack from "@/components/TileStack.vue";

const { data: stack } = useQuery({
  queryKey: ['get_stack'],
  queryFn: async (): Promise<Tile[]> => {
    return await invoke("get_tile_stack");
  },
  initialData: []
})

const { data: board } = useQuery({
  queryKey: ['get_board'],
  queryFn: async (): Promise<Tile[]> => {
    return await invoke("get_game_board");
  },
  initialData: []
})
</script>

<template>
  <main class="w-screen overflow-x-hidden">
    <div class="h-screen p-12">
      <GameGrid :tiles="board" class="max-h-full" />
    </div>
    <div class="p-12 flex flex-col gap-12">
      <TileStack :tiles="stack" />
      <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ board }}</p>
      <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ stack }}</p>
    </div>
  </main>
</template>
