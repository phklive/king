export interface PlayerStats {
  balance: number;
  playstyle: string;
}

export interface Player {
  name: string;
  image: string;
  lore: string;
  stats: PlayerStats;
}
