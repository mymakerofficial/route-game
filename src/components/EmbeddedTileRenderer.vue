<script setup lang="ts">
import {RawTile} from "@/types.ts";
import {useTranslateConnections} from "@/composables/useTranslateConnections.ts";
import {useTilePaths} from "@/composables/useTilePaths.ts";
import {SVGAttributes} from "vue";

const {
  tile,
  tileSize = 180,
  transform,
} = defineProps<{
  tile: RawTile
  tileSize?: number
  transform?: SVGAttributes["transform"]
}>()

const translatedConnections = useTranslateConnections(() => tile.connections)
const paths = useTilePaths(translatedConnections, tileSize)
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