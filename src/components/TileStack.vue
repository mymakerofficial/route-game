<script setup lang="ts">
import TileRenderer from "@/components/TileRenderer.vue";
import {Tile} from "@/types.ts";

defineProps<{
  tiles: Tile[];
}>()
</script>

<template>
  <div class="flex flex-wrap gap-y-1 mx-2">
    <div v-for="(tile, index) in tiles" :key="index" class="-mx-2 border-4 border-background rounded-xl">
      <TileRenderer :connections="tile.connections" class="w-12 md:w-16 xl:w-24" />
    </div>
  </div>
</template>
