export class Point {
  constructor(
    public x: number,
    public y: number
  ) {}

  public withX(x: number): Point {
    return new Point(x, this.y);
  }

  public withY(y: number): Point {
    return new Point(this.x, y)
  }

  public inset(n: number) {
    return new Point(this.x + n, this.y + n);
  }

  public offset(n: number) {
    return new Point(this.x - n, this.y - n);
  }

  public scale(scale: number) {
    return new Point(this.x * scale, this.y * scale);
  }
}