import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";
import {getPlayersQueryKey} from "@/composables/queries/useGetPlayers.ts";
import {PlayerTilePointer} from "@/types.ts";
import {getGameStateQueryKey} from "@/composables/queries/useGetGameState.ts";

export function useRotateTile(props: PlayerTilePointer) {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      return await invoke("rotate_player_tile", props);
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: getGameStateQueryKey() });
    }
  })
}