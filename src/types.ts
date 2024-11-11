export type RawConnection = [number, number];
export type RawConnections = [RawConnection, RawConnection, RawConnection, RawConnection];

export type RawTile = {
  isEmpty: boolean;
  connections: RawConnections;
}

export type RawPlayer = {
  positionOnBoard: number;
  positionOnTile: number;
  tileStack: RawTile[];
}

export type PlayerTilePointer = {
  playerIndex: number;
  tileIndex: number;
}