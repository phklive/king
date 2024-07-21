import React from "react";
import { Player } from "../types";

interface PlayerCardProps extends Player {
  value: number;
  onChange: (name: string, newValue: number) => void;
  winner: boolean;
}

const PlayerCard: React.FC<PlayerCardProps> = ({
  name,
  image,
  lore,
  stats,
  value,
  winner,
  onChange,
}) => {
  const getEmoji = (name: string) => {
    switch (name) {
      case "Analyst":
        return "📊";
      case "Degen":
        return "🤪";
      default:
        return "💰";
    }
  };

  return (
    <div
      className={`flex flex-col items-center flex-1 rounded border border-black bg-black ${winner && "shadow-2xl shadow-black border-yellow-400"} max-w-sm py-3`}
    >
      <h1 className="text-2xl rounded-xl mb-4">
        {name} {getEmoji(name)}
      </h1>
      {image && (
        <img
          src={`/images/${image}`}
          alt={`${name}'s image`}
          className="w-full h-[600px] mb-4"
        />
      )}
      <p className="m-8 text-center">{lore}</p>
      <div className="text-center mb-4">
        <p>{stats.balance} Ether</p>
        <p>{stats.playstyle} play-style</p>
      </div>
      <div className="w-full px-4">
        <div className="flex items-center justify-between">
          <label htmlFor={`${name}-input`}>Pick number of agents (0-10):</label>
          <input
            type="number"
            min="0"
            max="10"
            value={value}
            onChange={(e) =>
              onChange(
                name,
                Math.min(10, Math.max(0, parseInt(e.target.value) || 0)),
              )
            }
            className="w-16 text-center text-black rounded"
          />
        </div>
      </div>
    </div>
  );
};

export default PlayerCard;
