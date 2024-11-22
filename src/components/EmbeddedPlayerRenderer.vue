<script setup lang="ts">
import {RawPlayer} from "@/types.ts";
import {computed} from "vue";
import {Position} from "@/lib/position.ts";

const {
  player,
  index,
  tileSize = 180
} = defineProps<{
  player: RawPlayer
  index: number
  tileSize?: number
}>()

const size = computed(() => tileSize * 0.1)

const transform = computed(() => {
  return Position
      .fromRaw(player)
      .toPoint()
      .scale(tileSize)
      .toTransform()
})
</script>

<template>
  <g :transform="transform">
    <circle
      :r="size * 1.1"
      :class="[
        'fill-yellow-600',
        'fill-blue-600',
        'fill-fuchsia-600',
        'fill-lime-600',
        'fill-orange-600',
        'fill-rose-600',
      ][index % 6]"
    />
    <circle
      :r="size * 0.9"
      :class="[
        'fill-yellow-400',
        'fill-blue-400',
        'fill-fuchsia-400',
        'fill-lime-400',
        'fill-orange-400',
        'fill-rose-400',
      ][index % 6]"
    />
  </g>
</template>