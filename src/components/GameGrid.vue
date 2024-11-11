<script setup lang="ts">
import {RawPlayer, RawTile} from "@/types.ts";
import {computed, HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import EmbeddedTileRenderer from "@/components/EmbeddedTileRenderer.vue";
import {Point} from "@/lib/point.ts";
import EmbeddedPlayerRenderer from "@/components/EmbeddedPlayerRenderer.vue";

const {
  tiles,
  players,
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

function getTileTransform(position: number): string {
  return Point.fromPositionOnBoard(position).scale(tileSize.value).toTransform()
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
      <EmbeddedTileRenderer :tile="tile" :transform="getTileTransform(index)" :tile-size="tileSize" />
    </template>
    <template v-for="(player, index) in players" :key="index">
      <EmbeddedPlayerRenderer :player="player" :tile-size="tileSize" />
    </template>
  </svg>
</template>
