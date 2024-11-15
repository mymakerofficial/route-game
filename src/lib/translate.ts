import {Point} from "@/lib/point.ts";
import {Position} from "@/lib/position.ts";

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

function doRow(row: number, g: number) {
  return Array.from({ length: 12 }, (_, i) => {
    const positionOnBoard = Math.floor(i / 2) + (row * 6)
    const positionOnTile = g + i % 2
    return Position.fromRaw({ positionOnBoard, positionOnTile })
  });
}

function doCol(col: number, g: number) {
  return Array.from({ length: 12 }, (_, i) => {
    const positionOnBoard = col + Math.floor(i / 2) * 6
    const positionOnTile = g + i % 2
    return Position.fromRaw({ positionOnBoard, positionOnTile })
  })
}

export function getNotches() {
  return [
    ...doRow(0, 4),
    ...doCol(0, 6),
    ...doCol(5, 2),
    ...doRow(5, 0),
  ]
}