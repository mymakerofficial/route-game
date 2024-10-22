<script setup lang="ts">
import {Button} from "@/components/ui/button";
import TileRenderer from "@/components/TileRenderer.vue";
import {RotateCcwIcon} from "lucide-vue-next";
import {Tile} from "@/types.ts";
import {useRotateTile} from "@/composables/mutations/useRotateTile.ts";

const props = defineProps<{
  tile: Tile;
  playerIndex: number;
  tileIndex: number;
}>()

const { mutateAsync: handleRotate } = useRotateTile(props)
</script>

<template>
  <div>
    <div class="flex justify-end">
      <Button @click="handleRotate" variant="ghost" size="icon" class="size-7">
        <RotateCcwIcon class="size-5" />
      </Button>
    </div>
    <TileRenderer :connections="tile.connections" class="w-12 md:w-16 xl:w-24" />
  </div>
</template>