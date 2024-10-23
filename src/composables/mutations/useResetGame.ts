import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";

export function useResetGame() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      return await invoke("reset_game_state");
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries();
    }
  })
}