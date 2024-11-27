import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";
import {getGameStateQueryKey} from "@/composables/queries/useGetGameState.ts";

export function useResetGame() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      return await invoke("reset_game_state");
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: getGameStateQueryKey() });
    }
  })
}