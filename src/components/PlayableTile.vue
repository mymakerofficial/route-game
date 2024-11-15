<script setup lang="ts">
import {Button} from "@/components/ui/button";
import TileRenderer from "@/components/TileRenderer.vue";
import {ArrowUpFromLineIcon, RotateCcwIcon} from "lucide-vue-next";
import {RawTile} from "@/types.ts";
import {useRotateTile} from "@/composables/mutations/useRotateTile.ts";
import {usePlacePlayerTile} from "@/composables/mutations/usePlacePlayerTile.ts";

const { tile, ...playerTilePointer } = defineProps<{
  tile: RawTile;
  playerIndex: number;
  tileIndex: number;
}>()

const { mutateAsync: handleRotate } = useRotateTile(playerTilePointer)
const { mutateAsync: handlePlace } = usePlacePlayerTile({ ...playerTilePointer, positionOnBoard: 0 })
</script>

<template>
  <div >
    <div class="flex justify-end gap-1">
      <Button @click="handlePlace" variant="ghost" size="icon" class="h-7 flex-1">
        <ArrowUpFromLineIcon class="size-5" />
      </Button>
      <Button @click="handleRotate" variant="ghost" size="icon" class="h-7 flex-1">
        <RotateCcwIcon class="size-5" />
      </Button>
    </div>
    <TileRenderer
      :tile="tile"
      class="w-12 md:w-16 xl:w-24"
    />
  </div>
</template>