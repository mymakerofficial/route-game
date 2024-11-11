<script setup lang="ts">
import {Button} from "@/components/ui/button";
import TileRenderer from "@/components/TileRenderer.vue";
import {HandIcon, GrabIcon, RotateCcwIcon} from "lucide-vue-next";
import {RawTile} from "@/types.ts";
import {useRotateTile} from "@/composables/mutations/useRotateTile.ts";
import {useGrabTile} from "@/composables/useGrabTile.ts";

const { tile, ...playerTilePointer } = defineProps<{
  tile: RawTile;
  playerIndex: number;
  tileIndex: number;
}>()

const { mutateAsync: handleRotate } = useRotateTile(playerTilePointer)
const { grabTile, isGrabbedTile } = useGrabTile(playerTilePointer)
</script>

<template>
  <div :data-grabbed="isGrabbedTile" class="group">
    <div class="flex justify-end gap-1">
      <Button @click="grabTile" variant="ghost" size="icon" class="size-7 flex-1 group-data-[grabbed=true]:text-yellow-400 group-data-[grabbed=true]:bg-yellow-100">
        <GrabIcon v-if="isGrabbedTile" class="size-5" />
        <HandIcon v-else class="size-5" />
      </Button>
      <Button @click="handleRotate" variant="ghost" size="icon" class="size-7">
        <RotateCcwIcon class="size-5" />
      </Button>
    </div>
    <TileRenderer
      :tile="tile"
      class="w-12 md:w-16 xl:w-24 group-data-[grabbed=true]:scale-95 group-data-[grabbed=true]:border-yellow-300 group-data-[grabbed=true]:stroke-yellow-200 group-data-[grabbed=true]:border-4"
    />
  </div>
</template>