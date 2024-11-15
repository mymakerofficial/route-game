import {getGameBoardQueryKey} from "@/composables/queries/useGetGameBoard.ts";
import {getTileStackQueryKey} from "@/composables/queries/useGetTileStack.ts";
import {getPlayersQueryKey} from "@/composables/queries/useGetPlayers.ts";
import {PlayerTilePointer} from "@/types.ts";
import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";

type PlacePlayerTileProps = PlayerTilePointer & {
  positionOnBoard: number;
}

export function usePlacePlayerTile(props: PlacePlayerTileProps) {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      return await invoke("place_player_tile", props);
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: getPlayersQueryKey() });
      await queryClient.invalidateQueries({ queryKey: getGameBoardQueryKey() });
      await queryClient.invalidateQueries({ queryKey: getTileStackQueryKey() });
    }
  })
}