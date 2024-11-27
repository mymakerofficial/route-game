<script setup lang="ts">
import GameGrid from "@/components/GameGrid.vue";
import TileStack from "@/components/TileStack.vue";
import {Button} from "@/components/ui/button";
import {useGetGameState} from "@/composables/queries/useGetGameState.ts";
import {useResetGame} from "@/composables/mutations/useResetGame.ts";
import {useToggle} from "@vueuse/core";
import {ArrowUpFromLineIcon, EyeIcon, InfoIcon, RotateCcwIcon, SquareAsteriskIcon, UndoIcon} from "lucide-vue-next";
import PlayerControls from "@/components/PlayerControls.vue";

const { data: state } = useGetGameState()
const { mutateAsync: resetGame } = useResetGame()

const [showNerdStuff, toggleNerdStuff] = useToggle()
</script>

<template>
  <main class="p-12 flex flex-col gap-12">
    <div class="grid grid-cols-2 gap-12">
      <div>
        <GameGrid :state="state" class="max-w-screen-md" />
      </div>
      <div class="flex flex-col gap-6">
        <div v-if="state.players.length" class="flex flex-wrap gap-4">
          <PlayerControls
            :player="state.players[state.playerTurn]"
            :player-index="state.playerTurn"
            :key="state.playerTurn"
          />
        </div>
        <div v-if="state.players.length" class="flex gap-3 items-center p-3 font-medium bg-neutral-100 rounded-md">
          <SquareAsteriskIcon class="size-5" />
          <p>{{ state.tileStack.length }} tiles left.</p>
        </div>
        <div v-if="!state.players.length" class="flex gap-3 items-center p-3 font-medium bg-neutral-100 rounded-md">
          <InfoIcon class="size-5" />
          <p>Click the notches at the edge of the board to add players.</p>
        </div>
        <div v-else class="flex gap-3 items-center p-3 font-medium bg-neutral-100 rounded-md">
          <InfoIcon class="size-5" />
          <p>Click <ArrowUpFromLineIcon class="size-4 inline" /> to play the tile. Click <RotateCcwIcon class="size-4 inline" /> to rotate the tile.</p>
        </div>
        <div class="flex gap-4">
          <Button @click="resetGame" class="w-min gap-3">
            <UndoIcon class="size-5" />
            <span>Restart Game</span>
          </Button>
          <Button @click="() => toggleNerdStuff()" class="w-min gap-3" variant="ghost">
            <EyeIcon class="size-5" />
            <span>Toggle Nerd Stuff</span>
          </Button>
        </div>
        <div v-if="showNerdStuff" class="space-y-2">
          <h3 class="font-medium">Tile Stack</h3>
          <TileStack :tiles="state.tileStack" />
        </div>
      </div>
    </div>
    <p v-if="showNerdStuff" class="p-2 bg-neutral-200 rounded-md font-mono">{{ state }}</p>
  </main>
</template>
