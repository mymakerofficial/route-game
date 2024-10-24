<script setup lang="ts">
import {Tile} from "@/types.ts";
import {computed, HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import EmbeddedTileRenderer from "@/components/EmbeddedTileRenderer.vue";
import {positionOnBoardToPoint} from "@/lib/translate.ts";

const {
  tiles,
  size = 1000,
  ...props
} = defineProps<{
  tiles: Tile[]
  size?: number
  class?: HTMLAttributes["class"]
}>()

const tileSize = computed(() => {
  return size / 6
})

function getTransform(index: number): string {
  const { x, y } = positionOnBoardToPoint(index).scale(tileSize.value)
  return `translate(${x}, ${y})`
}
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${size} ${size}`"
    :class="cn('aspect-square border border-border', props.class)"
  >
    <template v-for="(tile, index) in tiles" :key="index">
      <!--      <BoardTilePlaceholder v-if="tile.isEmpty" :position-on-board="index" />-->
      <EmbeddedTileRenderer :tile="tile" :transform="getTransform(index)" :size="tileSize" />
    </template>
  </svg>
</template>
