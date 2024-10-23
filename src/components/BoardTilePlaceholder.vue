<script setup lang="ts">
import {usePlaceTile} from "@/composables/useGrabTile.ts";
import TileRenderer from "@/components/TileRenderer.vue";

const { positionOnBoard } = defineProps<{
  positionOnBoard: number
}>()

const { placeTile } = usePlaceTile(positionOnBoard)
</script>

<template>
  <button
    @click="placeTile"
    class="aspect-square rounded-lg flex items-center justify-center border-4 border-current hover:text-yellow-300 focus-visible:outline-none focus-visible:text-yellow-300 bg-stone-900 cursor-pointer"
  />
</template>