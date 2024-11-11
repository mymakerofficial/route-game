import {positionOnBoardToPoint, positionOnTileToPoint} from "@/lib/translate.ts";

export class Point {
  constructor(
    public x: number,
    public y: number
  ) {}

  static fromPositionOnBoard(position: number): Point {
    return positionOnBoardToPoint(position)
  }

  static fromPositionOnTile(position: number): Point {
    return positionOnTileToPoint(position)
  }

  static get Zero() {
    return new Point(0, 0);
  }

  public translateToBoard(): Point {
    return this.offset(1).scale(0.5)
  }

  public withX(x: number): Point {
    return new Point(x, this.y);
  }

  public withY(y: number): Point {
    return new Point(this.x, y)
  }

  public offset(n: number) {
    return new Point(this.x + n, this.y + n);
  }

  public add(point: Point) {
    return new Point(this.x + point.x, this.y + point.y);
  }

  public scale(scale: number) {
    return new Point(this.x * scale, this.y * scale);
  }

  // scales whichever axis is greater
  public scaleDominantAxis(scale: number) {
    if (Math.abs(this.x) > Math.abs(this.y)) {
      return new Point(this.x * scale, this.y);
    } else if (Math.abs(this.x) < Math.abs(this.y)) {
      return new Point(this.x, this.y * scale);
    } else {
      return this.scale(scale);
    }
  }

  public toTransform() {
    return `translate(${this.x}, ${this.y})`;
  }
}