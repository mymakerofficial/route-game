import {computed, MaybeRefOrGetter, toValue} from "vue";
import {Connection} from "@/lib/connection.ts";

export function useTilePaths(connections: MaybeRefOrGetter<Connection>, scale: MaybeRefOrGetter<number>): string {
  return computed(() => {
    return toValue(connections).map((connection) => {
      return connection.toSvgPath(toValue(scale))
    })
  })
}