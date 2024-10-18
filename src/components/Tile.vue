<script setup lang="ts">
import {Connections} from "../types.ts";
import {useTranslateConnections} from "../composables/useTranslateConnections.ts";
import {useTilePaths} from "../composables/useTilePaths.ts";

const props = defineProps<{
  connections: Connections;
}>();

const SIZE = 180;
const SCALE = SIZE / 3;

const bgColor = '#431407'
const borderColor = '#78350f'
const strokeDark = '#ca8a04'
const strokeLight = '#fed7aa'

const translatedConnections = useTranslateConnections(() => props.connections)
const paths = useTilePaths(translatedConnections, SCALE)
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${SIZE} ${SIZE}`"
    :style="{
      background: bgColor,
      border: `solid 2px ${borderColor}`,
      boxSizing: 'border-box',
      borderRadius: '4px',
      width: '100%',
      height: '100%'
    }"
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