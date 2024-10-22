<script setup lang="ts">
import {Connections, Tile} from "@/types.ts";
import {useTranslateConnections} from "@/composables/useTranslateConnections.ts";
import {useTilePaths} from "@/composables/useTilePaths.ts";
import colors from "tailwindcss/colors";
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";

const { tile, ...props } = defineProps<{
  tile: Tile;
  class?: HTMLAttributes["class"];
}>();

const SIZE = 180;
const SCALE = SIZE / 3;

const bgColor = colors.stone["800"]
const strokeDark = colors.stone["400"]
const strokeLight = colors.stone["50"]

const translatedConnections = useTranslateConnections(() => tile.connections)
const paths = useTilePaths(translatedConnections, SCALE)
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${SIZE} ${SIZE}`"
    :class="cn('bg-stone-800 border-2 border-stone-900 rounded-lg aspect-square', props.class)"
  >
    <template
      v-if="!tile.isEmpty"
      v-for="(path, index) in paths"
      :key="index"
    >
      <path
        :d="path"
        :stroke="bgColor"
        stroke-width="20"
        fill="transparent"
      />
      <path
        :d="path"
        :stroke="strokeDark"
        stroke-width="8"
        fill="transparent"
      />
      <path
        :d="path"
        :stroke="strokeLight"
        stroke-width="3"
        fill="transparent"
      />
    </template>
  </svg>
</template>