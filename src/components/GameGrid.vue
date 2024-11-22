<script setup lang="ts">
import {RawPlayer, RawTile} from "@/types.ts";
import {computed, HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import EmbeddedTileRenderer from "@/components/EmbeddedTileRenderer.vue";
import EmbeddedPlayerRenderer from "@/components/EmbeddedPlayerRenderer.vue";
import {getNotches} from "@/lib/translate.ts";
import {useAddPlayer} from "@/composables/mutations/useAddPlayer.ts";

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

const notches = computed(() => {
  if (tiles.some((tile) => !tile.isEmpty)) {
    return []
  }
  return getNotches().filter((notch) => !players.some((player) => notch.equals(player)))
})

const { mutate: addPlayer } = useAddPlayer()
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="`0 0 ${size} ${size}`"
    :class="cn('aspect-square border-4 border-stone-900 rounded-lg bg-stone-900', props.class)"
  >
    <template v-for="(tile, index) in tiles" :key="index">
      <EmbeddedTileRenderer :tile="tile" :position="index" :tile-size="tileSize" />
    </template>
    <template v-for="(position, index) in notches" :key="index">
      <rect
        :width="tileSize * 0.2"
        :height="tileSize * 0.2"
        :rx="tileSize * 0.05"
        :stroke-width="tileSize * 0.04"
        :transform="position.toPoint(0.9).scale(tileSize).offset(-tileSize * 0.1).toTransform()"
        class="fill-stone-100 stroke-stone-900 cursor-pointer"
        role="button"
        @click="() => addPlayer(position)"
      />
    </template>
    <template v-for="(player, index) in players" :key="index">
      <EmbeddedPlayerRenderer :player="player" :index="index" :tile-size="tileSize" />
    </template>
  </svg>
</template>
