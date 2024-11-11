import {Point} from "@/lib/point.ts";
import {RawConnection} from "@/types.ts";

export class Connection {
  constructor(
    public from: Point,
    public to: Point
  ) {}

  static fromPoints(from: Point, to: Point): Connection {
    return new Connection(from, to);
  }

  static fromRawConnection(connection: RawConnection): Connection {
    const [from, to] = connection;
    return Connection.fromPoints(Point.fromPositionOnTile(from), Point.fromPositionOnTile(to));
  }

  public translateToBoard(): Connection {
    return new Connection(this.from.translateToBoard(), this.to.translateToBoard());
  }

  public scale(scale: number): Connection {
    return new Connection(this.from.scale(scale), this.to.scale(scale));
  }

  public scaleDominantAxis(scale: number): Connection {
    return new Connection(this.from.scaleDominantAxis(scale), this.to.scaleDominantAxis(scale));
  }

  public toSvgPath(scale: number): string {
    const { from: fromHandle, to: toHandle } = this
      .scaleDominantAxis(0.333)
      .translateToBoard()
      .scale(scale);
    const { from, to } = this
      .translateToBoard()
      .scale(scale);
    return `M ${from.x} ${from.y} C ${fromHandle.x} ${fromHandle.y}, ${toHandle.x} ${toHandle.y}, ${to.x} ${to.y}`
  }
}