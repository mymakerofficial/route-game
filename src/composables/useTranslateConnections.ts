import {Connection, Connections, TranslatedConnection, TranslatedConnections} from "@/types.ts";
import {numberToPoint} from "@/composables/useNumberToPoint.ts";
import {computed, MaybeRefOrGetter, toValue} from "vue";

export function translateConnection(connection: Connection): TranslatedConnection {
  const [from, to] = connection;
  return { from: numberToPoint(from), to: numberToPoint(to) };
}

export function translateConnections(connections: Connections): TranslatedConnections {
  return connections.map((it) => translateConnection(it));
}

export function useTranslateConnections(connections: MaybeRefOrGetter<Connections>) {
  return computed(() => translateConnections(toValue(connections)));
}