import {Connection, Connections, TranslatedConnection, TranslatedConnections} from "@/types.ts";
import {numberToPoint} from "@/composables/useNumberToPoint.ts";
import {computed, MaybeRefOrGetter, toValue} from "vue";
import {translateConnections} from "@/lib/translate.ts";

export function useTranslateConnections(connections: MaybeRefOrGetter<Connections>) {
  return computed(() => translateConnections(toValue(connections)));
}