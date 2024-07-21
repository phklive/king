use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{agent::Agent, game::Game, types::Strategy};

pub type Summaries = Vec<Summary>;

/// Represents the averages of the execution on n games
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FinalSummary {
    strategy_summaries: Vec<StrategySummary>,
    times_played: u64,
    global_average_balance: u64,
    global_average_block: u64,
    global_average_times_played: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StrategySummary {
    strategy: Strategy,
    wins: u64,
    average_balance: u64,
    average_times_played: u64,
    average_block: u64,
}

/// Represents the execution of 1 game
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Summary {
    king: Agent,
    times_played: u64,
    block: u64,
    balance: u64,
}

impl Summary {
    pub fn new(game: &mut Game) -> Self {
        // Get account of winner
        let king_address = game.get_king().unwrap();

        // Find matching Agent
        let king = game
            .agents()
            .iter()
            .find(|agent| *agent.address() == king_address)
            .unwrap()
            .to_owned();

        // Get last block
        let block = game.get_current_block();

        // Get balance of winner
        let balance = game.get_account_balance(king_address);

        // Decrement by 1 for the last `pay_out` call
        let times_played = game.get_account_nonce(king_address) - 1;

        Self {
            king,
            times_played,
            block,
            balance,
        }
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Summary:\n\
             King: {:?}\n\
             Times Played: {}\n\
             Block: {}\n\
             Balance: {}",
            self.king, self.times_played, self.block, self.balance
        )
    }
}

impl FinalSummary {
    pub fn new(summaries: Summaries) -> Self {
        let times_played: u64 = summaries.len().try_into().unwrap();

        let strategy_types = [Strategy::Analyst, Strategy::Degen, Strategy::Whale];
        let mut strategy_summaries = Vec::new();

        for strategy in strategy_types {
            let summary = StrategySummary::new(summaries.clone(), strategy);
            strategy_summaries.push(summary)
        }

        let mut average_balance = 0;
        let mut average_block = 0;
        let mut average_times_played = 0;

        for ss in strategy_summaries.clone() {
            average_balance += ss.average_balance;
            average_block += ss.average_block;
            average_times_played += ss.average_times_played;
        }

        FinalSummary {
            times_played,
            strategy_summaries,
            global_average_balance: average_balance / times_played,
            global_average_block: average_block / times_played,
            global_average_times_played: average_times_played / times_played,
        }
    }
}

impl Display for FinalSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Final Summary:")?;
        writeln!(f, "Total Games Played: {}", self.times_played)?;
        writeln!(f, "Global Average Balance: {}", self.global_average_balance)?;
        writeln!(f, "Global Average Block: {}", self.global_average_block)?;
        writeln!(
            f,
            "Global Average Times Played: {}",
            self.global_average_times_played
        )?;
        writeln!(f, "\nStrategy Summaries:")?;
        for (index, summary) in self.strategy_summaries.iter().enumerate() {
            if index > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", summary)?;
        }
        Ok(())
    }
}

impl StrategySummary {
    pub fn new(summaries: Summaries, strategy: Strategy) -> Self {
        let mut block = 0;
        let mut balance = 0;
        let mut times_played = 0;

        let strategy_summaries: Summaries = summaries
            .iter()
            .filter(|summary| *summary.king.strategy() == strategy)
            .cloned()
            .collect();

        let wins = strategy_summaries.len().try_into().unwrap();

        for ss in strategy_summaries {
            block += ss.block;
            balance += ss.balance;
            times_played += ss.times_played;
        }

        StrategySummary {
            strategy,
            wins,
            average_block: if wins > 0 { block / wins } else { 0 },
            average_balance: if wins > 0 { balance / wins } else { balance },
            average_times_played: if wins > 0 {
                times_played / wins
            } else {
                times_played
            },
        }
    }
}

impl Display for StrategySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Strategy: {:?}\n\
             Wins: {}\n\
             Average Balance: {}\n\
             Average Times Played: {}\n\
             Average Block: {}",
            self.strategy,
            self.wins,
            self.average_balance,
            self.average_times_played,
            self.average_block
        )
    }
}
