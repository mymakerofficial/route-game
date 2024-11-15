import {Point} from "@/lib/point.ts";
import {RawPosition} from "@/types.ts";

export class Position implements RawPosition {
  constructor(public positionOnBoard: number, public positionOnTile: number) {}

  static fromRaw(raw: RawPosition) {
    return new Position(raw.positionOnBoard, raw.positionOnTile)
  }

  toRaw(): RawPosition {
    return {
      positionOnBoard: this.positionOnBoard,
      positionOnTile: this.positionOnTile
    }
  }

  toPoint(offsetOnTile: number = 1) {
    const tilePoint = Point.fromPositionOnBoard(this.positionOnBoard)
    return Point
      .fromPositionOnTile(this.positionOnTile)
      .scaleDominantAxis(offsetOnTile)
      .translateToBoard()
      .add(tilePoint)
  }

  equals(other: RawPosition) {
    return this.positionOnBoard === other.positionOnBoard && this.positionOnTile === other.positionOnTile
  }
}