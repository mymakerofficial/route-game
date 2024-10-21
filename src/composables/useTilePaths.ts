import {Point, TranslatedConnection, TranslatedConnections} from "@/types.ts";
import {computed, MaybeRefOrGetter, toValue} from "vue";

// gets the handle, 90 degrees from the center of the tile
function getHandlePoint(point: Point): Point {
  if (point.x === 0) {
    return { ...point, x: 1 };
  }
  if (point.x === 3) {
    return { ...point, x: 2 };
  }
  if (point.y === 0) {
    return { ...point, y: 1 };
  }
  if (point.y === 3) {
    return { ...point, y: 2 };
  }
}

export function getConnectionPath(connection: TranslatedConnection, scale: number): string {
  const {from, to} = connection;
  const fromHandle = getHandlePoint(from);
  const toHandle = getHandlePoint(to);
  const x1 = from.x * scale;
  const y1 = from.y * scale;
  const x1h = fromHandle.x * scale;
  const y1h = fromHandle.y * scale;
  const x2h = toHandle.x * scale;
  const y2h = toHandle.y * scale;
  const x2 = to.x * scale;
  const y2 = to.y * scale;
  return `M ${x1} ${y1} C ${x1h} ${y1h}, ${x2h} ${y2h}, ${x2} ${y2}`
}

export function useTilePaths(connections: MaybeRefOrGetter<TranslatedConnections>, scale: number): string {
  return computed(() => {
    return toValue(connections).map((connection) => {
      return getConnectionPath(connection, scale)
    })
  })
}