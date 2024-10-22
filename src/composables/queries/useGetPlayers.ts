import {useQuery} from "@tanstack/vue-query";
import {Player} from "@/types.ts";
import {invoke} from "@tauri-apps/api/core";

export function getPlayersQueryKey() {
  return ['get_players'];
}

export function useGetPlayers() {
  return useQuery({
    queryKey: getPlayersQueryKey(),
    queryFn: async (): Promise<Player[]> => {
      return await invoke("get_players");
    },
    initialData: []
  })
}