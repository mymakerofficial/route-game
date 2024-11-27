import {useQuery} from "@tanstack/vue-query";
import {RawGameState, RawTile} from "@/types.ts";
import {invoke} from "@tauri-apps/api/core";

export function getGameStateQueryKey() {
  return ['get_game_state'];
}

export function useGetGameState() {
  return useQuery({
    queryKey: getGameStateQueryKey(),
    queryFn: async (): Promise<RawGameState> => {
      return await invoke("get_game_state");
    },
    initialData: {
      board: [],
      tileStack: [],
      players: [],
      playerTurn: 0
    }
  })
}