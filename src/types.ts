export type Connection = [number, number];
export type Connections = [Connection, Connection, Connection, Connection];
export type Point = { x: number, y: number };
export type TranslatedConnection = { from: Point, to: Point };
export type TranslatedConnections = [TranslatedConnection, TranslatedConnection, TranslatedConnection, TranslatedConnection];

export type Tile = {
  isEmpty: boolean;
  connections: Connections;
}

export type Player = {
  positionOnBoard: number;
  positionOnTile: number;
  tileStack: Tile[];
}