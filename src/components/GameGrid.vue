<script setup lang="ts">
import TileRenderer from "@/components/TileRenderer.vue";
import {Tile} from "@/types.ts";

defineProps<{
  tiles: Tile[];
}>()
</script>

<template>
  <div class="grid grid-cols-6 grid-rows-6 gap-0.5 p-1 bg-stone-950 rounded-xl h-full aspect-square">
    <TileRenderer v-for="(tile, index) in tiles" :key="index" :connections="tile.connections" />
  </div>
</template>
