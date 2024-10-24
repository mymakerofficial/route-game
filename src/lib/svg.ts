import {Point, TranslatedConnection} from "@/types.ts";

// gets the handle, 90 degrees from the center of the tile
function getBezierHandlePoint(point: Point): Point {
  if (point.x === 0) {
    return point.withX(1);
  }
  if (point.x === 3) {
    return point.withX(2);
  }
  if (point.y === 0) {
    return point.withY(1);
  }
  if (point.y === 3) {
    return point.withY(2);
  }
}

export function getConnectionPath(connection: TranslatedConnection, scale: number): string {
  const fromHandle = getBezierHandlePoint(connection.from).scale(scale);
  const toHandle = getBezierHandlePoint(connection.to).scale(scale);
  const from = connection.from.scale(scale);
  const to = connection.to.scale(scale);
  return `M ${from.x} ${from.y} C ${fromHandle.x} ${fromHandle.y}, ${toHandle.x} ${toHandle.y}, ${to.x} ${to.y}`
}