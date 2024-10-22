import {useQuery} from "@tanstack/vue-query";
import {Tile} from "@/types.ts";
import {invoke} from "@tauri-apps/api/core";

export function getTileStackQueryKey() {
  return ['get_tile_stack'];
}

export function useGetTileStack() {
  return useQuery({
    queryKey: getTileStackQueryKey(),
    queryFn: async (): Promise<Tile[]> => {
      return await invoke("get_tile_stack");
    },
    initialData: []
  })
}