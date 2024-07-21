import React from "react";
import { FinalSummary, Strategy } from "../types";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";

interface SummaryProps {
  data: FinalSummary;
  times: number;
}

interface ChartData {
  strategy: Strategy;
  [key: string]: number | Strategy;
}

interface ChartConfig {
  dataKey: keyof Omit<ChartData, "strategy">;
  fill: string;
  label: string;
}

const CHART_CONFIGS: ChartConfig[] = [
  { dataKey: "wins", fill: "#8884d8", label: "Wins" },
  { dataKey: "block", fill: "#82ca9d", label: "Average Block Victory" },
  { dataKey: "balance", fill: "#ffc658", label: "Average End Balance" },
  { dataKey: "timesPlayed", fill: "#ff7300", label: "Times Played" },
];

const Summary: React.FC<SummaryProps> = ({ data, times }) => {
  const chartData: ChartData[] = data.strategy_summaries.map((ss) => ({
    strategy: ss.strategy,
    wins: ss.wins,
    block: ss.average_block,
    balance: ss.average_balance,
    timesPlayed: ss.average_times_played,
  }));

  const renderChart = ({ dataKey, fill, label }: ChartConfig) => (
    <div key={dataKey} className="w-full md:w-1/2 p-2">
      <ResponsiveContainer width="100%" height={300}>
        <BarChart data={chartData}>
          <XAxis dataKey="strategy" />
          <YAxis />
          <Tooltip />
          <Legend />
          <Bar dataKey={dataKey} fill={fill} name={label} />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );

  return (
    <div className="summary">
      <h2 className="text-2xl font-bold mb-4">
        Summary{" "}
        {(times > 1 && `of ${times} runs:`) || (times == 1 && "of the run:")}
      </h2>
      <div className="flex flex-wrap -mx-2">
        {CHART_CONFIGS.map(renderChart)}
      </div>
    </div>
  );
};

export default Summary;
