import {getGameStateQueryKey} from "@/composables/queries/useGetGameState.ts";
import {PlayerTilePointer} from "@/types.ts";
import {useMutation, useQueryClient} from "@tanstack/vue-query";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "vue-sonner";

export function usePlacePlayerTile(props: PlayerTilePointer) {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      return await invoke("place_player_tile", props);
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: getGameStateQueryKey() });
    },
    onError: toast
  })
}