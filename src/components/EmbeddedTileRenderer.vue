<script setup lang="ts">
import {Tile} from "@/types.ts";
import {useTranslateConnections} from "@/composables/useTranslateConnections.ts";
import {useTilePaths} from "@/composables/useTilePaths.ts";
import {computed, SVGAttributes} from "vue";

const {
  tile,
  size = 180,
  transform,
} = defineProps<{
  tile: Tile
  size?: number
  transform?: SVGAttributes["transform"]
}>()

const scale = computed(() => {
  return size / 3
})

const translatedConnections = useTranslateConnections(() => tile.connections)
const paths = useTilePaths(translatedConnections, scale)
</script>

<template>
  <g :transform="transform">
    <rect
      :width="size"
      :height="size"
      :stroke-width="size * 0.04"
      :rx="size * 0.05"
      class="fill-stone-800 stroke-stone-900"
    />
    <g v-if="!tile.isEmpty" class="stroke-neutral-50">
      <template
        v-for="(path, index) in paths"
        :key="index"
      >
        <path
          :d="path"
          :stroke-width="size * 0.1"
          fill="transparent"
          class="stroke-stone-800"
        />
        <path
          :d="path"
          :stroke-width="size * 0.04"
          fill="transparent"
          class="opacity-50"
        />
        <path
          :d="path"
          :stroke-width="size * 0.018"
          fill="transparent"
        />
      </template>
    </g>
  </g>
</template>