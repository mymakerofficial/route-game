<script setup lang="ts">
import GameGrid from "@/components/GameGrid.vue";
import TileStack from "@/components/TileStack.vue";
import {Button} from "@/components/ui/button";
import PlayerTileStack from "@/components/PlayerTileStack.vue";
import {useGetPlayers} from "@/composables/queries/useGetPlayers.ts";
import {useGetTileStack} from "@/composables/queries/useGetTileStack.ts";
import {useGetGameBoard} from "@/composables/queries/useGetGameBoard.ts";
import {useResetGame} from "@/composables/mutations/useResetGame.ts";
import {useToggle} from "@vueuse/core";
import {InfoIcon, UndoIcon, EyeIcon, ArrowUpFromLineIcon, RotateCcwIcon, SquareAsteriskIcon} from "lucide-vue-next";

const { data: stack } = useGetTileStack()
const { data: board } = useGetGameBoard()
const { data: players } = useGetPlayers()
const { mutateAsync: resetGame } = useResetGame()

const [showNerdStuff, toggleNerdStuff] = useToggle()
</script>

<template>
  <main class="p-12 flex flex-col gap-12">
    <div class="grid grid-cols-2 gap-12">
      <div>
        <GameGrid :tiles="board" :players="players" class="max-w-screen-md" />
      </div>
      <div class="flex flex-col gap-6">
        <div class="flex flex-wrap gap-4">
          <div v-for="(player, index) in players" :key="index" class="space-y-2">
            <h3
              class="p-2 font-medium rounded-md"
              :class="[
                'bg-yellow-200 text-yellow-900',
                'bg-blue-200 text-blue-900',
                'bg-fuchsia-200 text-fuchsia-900',
                'bg-lime-200 text-lime-900',
                'bg-orange-200 text-orange-900',
                'bg-rose-200 text-rose-900',
              ][index % 6]"
            >
              Player {{ index }}
            </h3>
            <PlayerTileStack :tiles="player.tileStack" :player-index="index" />
          </div>
        </div>
        <div v-if="players.length" class="flex gap-3 items-center p-3 font-medium bg-neutral-100 rounded-md">
          <SquareAsteriskIcon class="size-5" />
          <p>{{ stack.length }} tiles left.</p>
        </div>
        <div v-if="!players.length" class="flex gap-3 items-center p-3 font-medium bg-neutral-100 rounded-md">
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
            <span>Toggle Tile Stack</span>
          </Button>
        </div>
        <div v-if="showNerdStuff" class="space-y-2">
          <h3 class="font-medium">Tile Stack</h3>
          <TileStack :tiles="stack" />
        </div>
      </div>
    </div>
    <p v-if="showNerdStuff" class="p-2 bg-neutral-200 rounded-md font-mono">{{ board }}</p>
    <p v-if="showNerdStuff" class="p-2 bg-neutral-200 rounded-md font-mono">{{ stack }}</p>
    <p v-if="showNerdStuff" class="p-2 bg-neutral-200 rounded-md font-mono">{{ players }}</p>
  </main>
</template>
