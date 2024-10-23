<script setup lang="ts">
import {Tile} from "@/types.ts";
import {useTranslateConnections} from "@/composables/useTranslateConnections.ts";
import {useTilePaths} from "@/composables/useTilePaths.ts";
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";

const { tile, ...props } = defineProps<{
  tile: Tile;
  class?: HTMLAttributes["class"];
}>()

const SIZE = 180;
const SCALE = SIZE / 3;

const translatedConnections = useTranslateConnections(() => tile.connections)
const paths = useTilePaths(translatedConnections, SCALE)
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${SIZE} ${SIZE}`"
    :class="cn('bg-stone-800 stroke-neutral-50 border-2 border-stone-900 rounded-lg aspect-square', props.class)"
  >
    <template
      v-if="!tile.isEmpty"
      v-for="(path, index) in paths"
      :key="index"
    >
      <path
        :d="path"
        stroke-width="20"
        fill="transparent"
        class="stroke-stone-800"
      />
      <path
        :d="path"
        stroke-width="8"
        fill="transparent"
        class="opacity-50"
      />
      <path
        :d="path"
        stroke-width="3"
        fill="transparent"
      />
    </template>
  </svg>
</template>