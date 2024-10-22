<script setup lang="ts">
import TileRenderer from "@/components/TileRenderer.vue";
import {Tile} from "@/types.ts";
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";

const props = defineProps<{
  tiles: Tile[];
  class?: HTMLAttributes["class"];
}>()
</script>

<template>
  <div :class="cn('grid grid-cols-6 grid-rows-6 gap-0.5 p-1 bg-stone-950 rounded-xl aspect-square', props.class)">
    <TileRenderer v-for="(tile, index) in tiles" :key="index" :tile="tile" class="size-full" />
  </div>
</template>
