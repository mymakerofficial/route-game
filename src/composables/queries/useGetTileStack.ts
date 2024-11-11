import {useQuery} from "@tanstack/vue-query";
import {RawTile} from "@/types.ts";
import {invoke} from "@tauri-apps/api/core";

export function getTileStackQueryKey() {
  return ['get_tile_stack'];
}

export function useGetTileStack() {
  return useQuery({
    queryKey: getTileStackQueryKey(),
    queryFn: async (): Promise<RawTile[]> => {
      return await invoke("get_tile_stack");
    },
    initialData: []
  })
}