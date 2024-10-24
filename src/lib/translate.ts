import {Connection, Connections, TranslatedConnection, TranslatedConnections} from "@/types.ts";
import {Point} from "@/lib/point.ts";

export function positionOnTileToPoint(position: number): Point {
  switch (position) {
    case 0: return new Point(1, 3);
    case 1: return new Point(2, 3);
    case 2: return new Point(3, 2);
    case 3: return new Point(3, 1);
    case 4: return new Point(2, 0);
    case 5: return new Point(1, 0);
    case 6: return new Point(0, 1);
    case 7: return new Point(0, 2);
  }
}

export function positionOnBoardToPoint(position: number): Point {
  const x = position % 6;
  const y = Math.floor(position / 6);
  return new Point(x, y)
}

export function translateConnection(connection: Connection): TranslatedConnection {
  const [from, to] = connection;
  return { from: positionOnTileToPoint(from), to: positionOnTileToPoint(to) };
}

export function translateConnections(connections: Connections): TranslatedConnections {
  return connections.map((it) => translateConnection(it));
}
