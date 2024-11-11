import {useQuery} from "@tanstack/vue-query";
import {RawPlayer} from "@/types.ts";
import {invoke} from "@tauri-apps/api/core";

export function getPlayersQueryKey() {
  return ['get_players'];
}

export function useGetPlayers() {
  return useQuery({
    queryKey: getPlayersQueryKey(),
    queryFn: async (): Promise<RawPlayer[]> => {
      return await invoke("get_players");
    },
    initialData: []
  })
}