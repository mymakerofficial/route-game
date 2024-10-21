<script setup lang="ts">
import {Connections} from "@/types.ts";
import {useTranslateConnections} from "@/composables/useTranslateConnections.ts";
import {useTilePaths} from "@/composables/useTilePaths.ts";
import colors from "tailwindcss/colors";

const props = defineProps<{
  connections: Connections;
}>();

const SIZE = 180;
const SCALE = SIZE / 3;

const bgColor = colors.stone["800"]
const strokeDark = colors.stone["400"]
const strokeLight = colors.stone["50"]

const translatedConnections = useTranslateConnections(() => props.connections)
const paths = useTilePaths(translatedConnections, SCALE)
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${SIZE} ${SIZE}`"
    class="bg-stone-800 border-2 border-stone-900 size-full rounded-lg"
  >
    <template
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