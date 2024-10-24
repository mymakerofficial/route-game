import {Point, TranslatedConnections} from "@/types.ts";
import {computed, MaybeRefOrGetter, toValue} from "vue";
import {getConnectionPath} from "@/lib/svg.ts";

export function useTilePaths(connections: MaybeRefOrGetter<TranslatedConnections>, scale: MaybeRefOrGetter<number>): string {
  return computed(() => {
    return toValue(connections).map((connection) => {
      return getConnectionPath(connection, toValue(scale))
    })
  })
}