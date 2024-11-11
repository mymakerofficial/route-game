import {RawConnections } from "@/types.ts";
import {Point} from "@/lib/point.ts";
import {Connection} from "@/lib/connection.ts";

export function positionOnTileToPoint(position: number): Point {
  switch (position) {
    case 0: return new Point(-0.333, 1);
    case 1: return new Point(0.333, 1);
    case 2: return new Point(1, 0.333);
    case 3: return new Point(1, -0.333);
    case 4: return new Point(0.333, -1);
    case 5: return new Point(-0.333, -1);
    case 6: return new Point(-1, -0.333);
    case 7: return new Point(-1, 0.333);
  }
}

export function positionOnBoardToPoint(position: number): Point {
  const x = position % 6;
  const y = Math.floor(position / 6);
  return new Point(x, y)
}

export function translateConnections(connections: RawConnections): Connection[] {
  return connections.map((it) => Connection.fromRawConnection(it));
}
