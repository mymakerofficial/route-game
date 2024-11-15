<script setup lang="ts">
import {RawTile} from "@/types.ts";
import {useConnectionsFromRawConnections} from "@/composables/useConnectionsFromRawConnections.ts";
import {useTilePaths} from "@/composables/useTilePaths.ts";
import {Point} from "@/lib/point.ts";
import {computed} from "vue";

const {
  tile,
  position = 0,
  tileSize = 180,
} = defineProps<{
  tile: RawTile
  position?: number
  tileSize?: number
}>()

const transform = computed(() => {
  return Point.fromPositionOnBoard(position).scale(tileSize).toTransform()
})

const connections = useConnectionsFromRawConnections(() => tile.connections)
const paths = useTilePaths(connections, tileSize)
</script>

<template>
  <g :transform="transform">
    <rect
      :width="tileSize"
      :height="tileSize"
      :stroke-width="tileSize * 0.04"
      :rx="tileSize * 0.05"
      class="fill-stone-800 stroke-stone-900"
    />
    <g v-if="!tile.isEmpty" class="stroke-neutral-50">
      <template
        v-for="(path, index) in paths"
        :key="index"
      >
        <path
          :d="path"
          :stroke-width="tileSize * 0.1"
          fill="transparent"
          class="stroke-stone-800"
        />
        <path
          :d="path"
          :stroke-width="tileSize * 0.04"
          fill="transparent"
          class="opacity-50"
        />
        <path
          :d="path"
          :stroke-width="tileSize * 0.018"
          fill="transparent"
        />
      </template>
    </g>
  </g>
</template>