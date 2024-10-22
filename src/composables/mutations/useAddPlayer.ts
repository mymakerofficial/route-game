import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";
import {getPlayersQueryKey} from "@/composables/queries/useGetPlayers.ts";
import {getTileStackQueryKey} from "@/composables/queries/useGetTileStack.ts";

export function useAddPlayer() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      return await invoke("add_player");
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: getPlayersQueryKey() });
      await queryClient.invalidateQueries({ queryKey: getTileStackQueryKey() });
    }
  })
}