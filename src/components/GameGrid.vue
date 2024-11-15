<script setup lang="ts">
import {RawPlayer, RawTile} from "@/types.ts";
import {computed, HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import EmbeddedTileRenderer from "@/components/EmbeddedTileRenderer.vue";
import EmbeddedPlayerRenderer from "@/components/EmbeddedPlayerRenderer.vue";
import {getNotches} from "@/lib/translate.ts";

const {
  tiles,
  size = 1000,
  ...props
} = defineProps<{
  tiles: RawTile[]
  players: RawPlayer[]
  size?: number
  class?: HTMLAttributes["class"]
}>()

const tileSize = computed(() => {
  return size / 6
})
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${size} ${size}`"
    :class="cn('aspect-square border border-border', props.class)"
  >
    <template v-for="(tile, index) in tiles" :key="index">
      <EmbeddedTileRenderer :tile="tile" :position="index" :tile-size="tileSize" />
    </template>
    <template v-for="(player, index) in players" :key="index">
      <EmbeddedPlayerRenderer :player="player" :tile-size="tileSize" />
    </template>
    <template v-for="(position, index) in getNotches()" :key="index">
      <rect
        :width="tileSize * 0.2"
        :height="tileSize * 0.2"
        :rx="tileSize * 0.05"
        :transform="position.toPoint(0.9).scale(tileSize).offset(-tileSize * 0.1).toTransform()"
        class="fill-neutral-100"
      />
    </template>
  </svg>
</template>
