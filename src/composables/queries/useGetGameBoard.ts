import {useQuery} from "@tanstack/vue-query";
import {Tile} from "@/types.ts";
import {invoke} from "@tauri-apps/api/core";

export function getGameBoardQueryKey() {
  return ['get_game_board'];
}

export function useGetGameBoard() {
  return useQuery({
    queryKey: getGameBoardQueryKey(),
    queryFn: async (): Promise<Tile[]> => {
      return await invoke("get_game_board");
    },
    initialData: []
  })
}