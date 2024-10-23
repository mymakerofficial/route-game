import {computed, ref} from "vue";
import {PlayerTilePointer} from "@/types.ts";
import {usePlacePlayerTile} from "@/composables/mutations/usePlacePlayerTile.ts";

const grabbedTilePlayerIndex = ref(-1)
const grabbedTileTileIndex = ref(-1)

function ungrabTile() {
  grabbedTilePlayerIndex.value = -1
  grabbedTileTileIndex.value = -1
}

function grabTile(playerTilePointer: PlayerTilePointer) {
  if (getIsGrabbedTile(playerTilePointer)) {
    ungrabTile()
    return
  }
  grabbedTilePlayerIndex.value = playerTilePointer.playerIndex
  grabbedTileTileIndex.value = playerTilePointer.tileIndex
}

function getIsGrabbedTile(playerTilePointer: PlayerTilePointer) {
  return playerTilePointer.playerIndex === grabbedTilePlayerIndex.value
    && playerTilePointer.tileIndex === grabbedTileTileIndex.value
}

function isGrabbedTile(playerTilePointer: PlayerTilePointer) {
  return computed(() => getIsGrabbedTile(playerTilePointer))
}

export function useGrabTile(playerTilePointer: PlayerTilePointer) {
  return {
    isGrabbedTile: isGrabbedTile(playerTilePointer),
    grabTile: () => grabTile(playerTilePointer),
  }
}

export function usePlaceTile(positionOnBoard: number) {
  const { mutateAsync: placePlayerTile } = usePlacePlayerTile()

  async function placeTile() {
    if (grabbedTilePlayerIndex.value === -1 || grabbedTileTileIndex.value === -1) {
      return
    }
    await placePlayerTile({
      playerIndex: grabbedTilePlayerIndex.value,
      tileIndex: grabbedTileTileIndex.value,
      positionOnBoard,
    })
    ungrabTile()
  }
  return { placeTile }
}