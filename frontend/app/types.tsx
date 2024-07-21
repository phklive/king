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

export type Strategy = "Analyst" | "Degen" | "Whale";

export interface StrategySummary {
  strategy: Strategy;
  wins: number;
  average_balance: number;
  average_times_played: number;
  average_block: number;
}

export interface FinalSummary {
  strategy_summaries: StrategySummary[];
  times_played: number;
  global_average_balance: number;
  global_average_block: number;
  global_average_times_played: number;
}
