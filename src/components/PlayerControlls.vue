<script setup lang="ts">
import PlayerTileStack from "@/components/PlayerTileStack.vue";
import {EyeOffIcon, EyeIcon, SkullIcon} from "lucide-vue-next";
import {RawPlayer} from "@/types.ts";
import {useToggle} from "@vueuse/core";
import {Button} from "@/components/ui/button";
import {computed} from "vue";

const {
  player
} = defineProps<{
  player: RawPlayer;
  playerIndex: number;
}>()

const [blur, toggleBlur] = useToggle(true)

const tiles = computed(() => {
  if (blur.value) {
    return player.tileStack.map(() => ({
      isEmpty: true,
      connections: [[0, 0], [0, 0], [0, 0], [0, 0]],
    }))
  }

  return player.tileStack
})
</script>

<template>
  <div class="space-y-2">
    <div class="flex gap-3 items-center">
      <div
          class="p-2 font-medium rounded-md flex gap-2 items-center"
          :class="[
        'bg-yellow-200 text-yellow-900',
        'bg-blue-200 text-blue-900',
        'bg-fuchsia-200 text-fuchsia-900',
        'bg-lime-200 text-lime-900',
        'bg-orange-200 text-orange-900',
        'bg-rose-200 text-rose-900',
      ][playerIndex % 6]"
      >
        <span>Player {{ playerIndex + 1 }}</span>
        <SkullIcon v-if="player.isDead" class="size-5" />
      </div>
      <Button @click="() => toggleBlur()" variant="ghost" class="w-min gap-3">
        <template v-if="blur">
          <EyeOffIcon class="size-5" />
          <span>Show</span>
        </template>
        <template v-else>
          <EyeIcon class="size-5" />
          <span>Hide</span>
        </template>
      </Button>
    </div>
    <PlayerTileStack :tiles="tiles" :player-index="playerIndex" />
  </div>
</template>
