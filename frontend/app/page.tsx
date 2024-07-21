"use client";
import React, { useState, useEffect } from "react";
import PlayerCard from "./components/player";
import { FinalSummary, Player } from "./types";
import Summary from "./components/summary";

export default function Home() {
  const [players, setPlayers] = useState<Player[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [playerValues, setPlayerValues] = useState<{ [key: string]: number }>(
    {},
  );
  const [summary, setSummary] = useState<FinalSummary | null>(null);
  const [times, setTimes] = useState(1);

  useEffect(() => {
    const fetchPlayers = async () => {
      try {
        const response = await fetch("http://localhost:8080/players");
        if (!response.ok) {
          throw new Error("Failed to fetch players");
        }
        const data: Player[] = await response.json();
        setPlayers(data);
        // Initialize player values
        const initialValues = data.reduce(
          (acc, player) => {
            acc[player.name] = 0;
            return acc;
          },
          {} as { [key: string]: number },
        );
        setPlayerValues(initialValues);
        setIsLoading(false);
      } catch (err) {
        setError("Failed to fetch players. Please try again later.");
        setIsLoading(false);
      }
    };
    fetchPlayers();
  }, []);

  const handlePlayerValueChange = (name: string, newValue: number) => {
    setPlayerValues((prev) => ({ ...prev, [name]: newValue }));
  };

  const tooFewPlayers = () => {
    return (
      Object.values(playerValues).reduce((sum, value) => sum + value, 0) <= 1
    );
  };

  const incorrectRunTimes = () => {
    return times < 1 || times > 1000;
  };

  const handleSubmit = async () => {
    setIsSubmitting(true);
    const formattedData = [Object.entries(playerValues), times];

    try {
      const response = await fetch("http://localhost:8080/play", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(formattedData),
      });

      if (!response.ok) {
        throw new Error("Failed to submit data");
      }

      const result = await response.json();
      setSummary(result as FinalSummary);
    } catch (error) {
      setError("Failed to start the game. Please try again.");
    } finally {
      setIsSubmitting(false);
    }
  };

  if (isLoading) {
    return <div>Loading players...</div>;
  }
  if (error) {
    return <div>Error: {error}</div>;
  }

  return (
    <main className="flex min-h-screen flex-col items-center sm:px-8 px-10 md:px-40 py-10">
      <h1 className="text-3xl">King of the Ether 👑</h1>
      <a
        href="https://github.com/phklive/king"
        className="hover:text-blue-500 underline"
      >
        github
      </a>
      <p className="my-10 text-center md:text-lg font-bold">
        Welcome to "King of the Ether 👑" a real-time game play-out simulation,
        under the hood a Rust backend spins up a{" "}
        <a
          href="https://github.com/bluealloy/revm"
          className="hover:text-blue-500 underline"
        >
          Revm
        </a>{" "}
        instance, deploys the game{" "}
        <a
          href="https://github.com/phklive/King/blob/main/backend/static/king.sol"
          className="hover:text-blue-500 underline"
        >
          smart contract
        </a>{" "}
        within, creates agents with different characteristics / strategies and
        executes the simulation until a new king is crowned 👑
      </p>
      <div className="flex flex-col md:flex-row justify-center gap-10">
        {players.map((player, index) => (
          <PlayerCard
            key={index}
            {...player}
            value={playerValues[player.name] || 0}
            onChange={handlePlayerValueChange}
          />
        ))}
      </div>
      <div className="flex flex-col items-center justify-between bg-black my-10 p-8 border border-black rounded-xl shadow-black shadow-2xl">
        <label htmlFor="times">Pick number of simulation runs (1-1000):</label>
        <input
          id="times"
          type="number"
          min="1"
          max="1000"
          value={times}
          onChange={(e) => setTimes(+e.target.value)}
          className="self-center text-center text-black rounded mt-4"
        />
        <button
          onClick={handleSubmit}
          disabled={tooFewPlayers() || isSubmitting || incorrectRunTimes()}
          className={`mt-4 font-bold py-2 px-4 rounded h-16 bg-white text-black ${
            tooFewPlayers() || isSubmitting || incorrectRunTimes()
              ? "bg-gray-600 cursor-not-allowed"
              : "bg-black hover:bg-blue-700"
          }`}
        >
          {isSubmitting ? "Loading..." : "Simulate"}
          <p>(2 agents minimum)</p>
        </button>
      </div>
      {summary && <Summary data={summary} times={times} />}
    </main>
  );
}
