export type RawConnection = [number, number];
export type RawConnections = [RawConnection, RawConnection, RawConnection, RawConnection];

export type RawTile = {
  isEmpty: boolean;
  connections: RawConnections;
}

export type RawPosition = {
  positionOnBoard: number;
  positionOnTile: number;
}

export type RawPlayer = RawPosition & {
  tileStack: RawTile[];
}

export type PlayerTilePointer = {
  playerIndex: number;
  tileIndex: number;
}