<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {Player, Tile} from "@/types.ts";
import {useMutation, useQuery, useQueryClient} from "@tanstack/vue-query";
import GameGrid from "@/components/GameGrid.vue";
import TileStack from "@/components/TileStack.vue";
import {Button} from "@/components/ui/button";
import PlayerTileStack from "@/components/PlayerTileStack.vue";
import {useGetPlayers} from "@/composables/queries/useGetPlayers.ts";
import {useAddPlayer} from "@/composables/mutations/useAddPlayer.ts";
import {useGetTileStack} from "@/composables/queries/useGetTileStack.ts";
import {useGetGameBoard} from "@/composables/queries/useGetGameBoard.ts";
import {useResetGame} from "@/composables/mutations/useResetGame.ts";
import {ref} from "vue";

const { data: stack } = useGetTileStack()
const { data: board } = useGetGameBoard()
const { data: players } = useGetPlayers()
const { mutateAsync: addPlayer } = useAddPlayer()
const { mutateAsync: resetGame } = useResetGame()
</script>

<template>
  <main class="p-12 flex flex-col gap-12">
    <div class="grid grid-cols-2 gap-12">
      <div>
        <GameGrid :tiles="board" class="max-w-screen-md" />
      </div>
      <div class="flex flex-col gap-12">
        <div class="space-y-2">
          <h3 class="font-medium">Tile Stack</h3>
          <TileStack :tiles="stack" />
        </div>
        <div class="flex gap-4">
          <Button @click="resetGame" class="w-min">Reset</Button>
          <Button @click="addPlayer" class="w-min">Add Player</Button>
        </div>
        <div class="flex flex-wrap gap-4">
          <div v-for="(player, index) in players" :key="index" class="space-y-2">
            <h3 class="font-medium">Player {{ index }}</h3>
            <PlayerTileStack :tiles="player.tileStack" :player-index="index" />
          </div>
        </div>
      </div>
    </div>
    <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ board }}</p>
    <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ stack }}</p>
    <p class="p-2 bg-neutral-200 rounded-md font-mono">{{ players }}</p>
  </main>
</template>
