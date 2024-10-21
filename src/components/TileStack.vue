<script setup lang="ts">
import TileRenderer from "@/components/TileRenderer.vue";
import {Tile} from "@/types.ts";

defineProps<{
  tiles: Tile[];
}>()
</script>

<template>
  <div class="flex flex-wrap gap-0.5">
    <TileRenderer v-for="(tile, index) in tiles" :key="index" :connections="tile.connections" class="w-1/12" />
  </div>
</template>
