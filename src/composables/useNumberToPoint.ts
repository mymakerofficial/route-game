import {computed, MaybeRefOrGetter, toValue} from "vue";
import {Point} from "@/types.ts";

export function numberToPoint(number: number): Point {
  switch (number) {
    case 0: return { x: 1, y: 3 };
    case 1: return { x: 2, y: 3 };
    case 2: return { x: 3, y: 2 };
    case 3: return { x: 3, y: 1 };
    case 4: return { x: 2, y: 0 };
    case 5: return { x: 1, y: 0 };
    case 6: return { x: 0, y: 1 };
    case 7: return { x: 0, y: 2 };
  }
}

export function useNumberToPoint(number: MaybeRefOrGetter<number>): Point {
  return computed(() => numberToPoint(toValue(number)));
}