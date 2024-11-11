<script setup lang="ts">
import {RawTile} from "@/types.ts";
import PlayableTile from "@/components/PlayableTile.vue";

defineProps<{
  tiles: RawTile[];
  playerIndex: number;
}>()
</script>

<template>
  <div class="flex flex-wrap gap-1">
    <PlayableTile
      v-for="(tile, index) in tiles"
      :key="index"
      :tile="tile"
      :tile-index="index"
      :player-index="playerIndex"
      class="w-12 md:w-16 xl:w-24"
    />
  </div>
</template>