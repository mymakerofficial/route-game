import {RawConnections} from "@/types.ts";
import {computed, MaybeRefOrGetter, toValue} from "vue";
import {Connection} from "@/lib/connection.ts";

export function useConnectionsFromRawConnections(connections: MaybeRefOrGetter<RawConnections>) {
  return computed(() => {
    return toValue(connections).map((it) => Connection.fromRawConnection(it))
  });
}