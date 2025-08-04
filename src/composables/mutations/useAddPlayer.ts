import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";
import {Position} from "@/lib/position.ts";
import {getGameStateQueryKey} from "@/composables/queries/useGetGameState.ts";

export function useAddPlayer() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (position: Position) => {
      await invoke("add_player", position.toRaw());
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: getGameStateQueryKey() });
    }
  })
}