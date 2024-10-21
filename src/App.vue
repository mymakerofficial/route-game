<script setup lang="ts">
import {ref} from "vue";
import { invoke } from "@tauri-apps/api/core";
import Tile from "@/components/Tile.vue";
import {Connections} from "@/types.ts";
import {Button} from "@/components/ui/button";

const tiles = ref<Connections[]>([]);

async function submit() {
  tiles.value = await invoke("get_possible_connections");
}
</script>

<template>
  <main class="container">
    <Button @click="submit">submit</Button>
    <p>{{ tiles }}</p>
    <div class="tile-container">
      <Tile v-for="(connections, index) in tiles" :key="index" :connections="connections" />
    </div>
  </main>
</template>

<style>
.tile-container {
  display: grid;
  grid-template-columns: repeat(6, minmax(0, 1fr));
  grid-template-rows: repeat(6, minmax(0, 1fr));
  max-height: 70vh;
  aspect-ratio: 1;
  gap: 0.1rem;
  padding: 0.1rem;
  background: #311202;
}
</style>