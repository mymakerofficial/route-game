<script setup lang="ts">
import {RawTile} from "@/types.ts";
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import EmbeddedTileRenderer from "@/components/EmbeddedTileRenderer.vue";

const { tile, size = 180, ...props } = defineProps<{
  tile: RawTile
  size?: number
  class?: HTMLAttributes["class"]
}>()
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${size} ${size}`"
    :class="cn('rounded-lg aspect-square', props.class)"
  >
    <EmbeddedTileRenderer :tile="tile" :size="size" />
  </svg>
</template>