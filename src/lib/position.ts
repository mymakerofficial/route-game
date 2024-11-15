import {Point} from "@/lib/point.ts";

export class Position {
  constructor(public positionOnBoard: number, public positionOnTile: number) {}

  static fromRaw(raw: {positionOnBoard: number, positionOnTile: number}) {
    return new Position(raw.positionOnBoard, raw.positionOnTile)
  }

  toPoint(offsetOnTile: number = 1) {
    const tilePoint = Point.fromPositionOnBoard(this.positionOnBoard)
    return Point
      .fromPositionOnTile(this.positionOnTile)
      .scaleDominantAxis(offsetOnTile)
      .translateToBoard()
      .add(tilePoint)
  }
}