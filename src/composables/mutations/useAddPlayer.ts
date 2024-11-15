import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";
import {getPlayersQueryKey} from "@/composables/queries/useGetPlayers.ts";
import {getTileStackQueryKey} from "@/composables/queries/useGetTileStack.ts";
import {Position} from "@/lib/position.ts";

export function useAddPlayer() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (position: Position) => {
      await invoke("add_player", position.toRaw());
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: getPlayersQueryKey() });
      await queryClient.invalidateQueries({ queryKey: getTileStackQueryKey() });
    }
  })
}