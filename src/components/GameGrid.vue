<script setup lang="ts">
import TileRenderer from "@/components/TileRenderer.vue";
import {Tile} from "@/types.ts";
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import BoardTilePlaceholder from "@/components/BoardTilePlaceholder.vue";

const props = defineProps<{
  tiles: Tile[];
  class?: HTMLAttributes["class"];
}>()
</script>

<template>
  <div :class="cn('grid grid-cols-6 grid-rows-6 gap-0.5 p-1 bg-stone-950 rounded-xl aspect-square', props.class)">
    <template v-for="(tile, index) in tiles" :key="index">
      <BoardTilePlaceholder v-if="tile.isEmpty" :position-on-board="index" />
      <TileRenderer v-else :tile="tile" />
    </template>
  </div>
</template>
