<script setup lang="ts">
import {RawPlayer} from "@/types.ts";
import {computed} from "vue";
import {Point} from "@/lib/point.ts";

const {
  player,
  tileSize = 180
} = defineProps<{
  player: RawPlayer
  tileSize?: number
}>()

const size = computed(() => tileSize * 0.1)

const transform = computed(() => {
  return Point
      .fromPositionOnTile(player.positionOnTile)
      .translateToBoard()
      .add(Point.fromPositionOnBoard(player.positionOnBoard))
      .scale(tileSize)
      .toTransform()
})
</script>

<template>
  <g :transform="transform">
    <circle
      :r="size * 1.1"
      class="fill-yellow-600"
    />
    <circle
      :r="size * 0.9"
      class="fill-yellow-400"
    />
  </g>
</template>