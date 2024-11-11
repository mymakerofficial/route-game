import {RawConnections} from "@/types.ts";
import {computed, MaybeRefOrGetter, toValue} from "vue";
import {translateConnections} from "@/lib/translate.ts";

export function useTranslateConnections(connections: MaybeRefOrGetter<RawConnections>) {
  return computed(() => translateConnections(toValue(connections)));
}