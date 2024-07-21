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
  const [king, setKing] = useState("");

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

  const handleSubmit = async () => {
    setIsSubmitting(true);
    const formattedData = Object.entries(playerValues);
    console.log(formattedData);

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
      console.log("Game started successfully:", result);
      setSummary(result as FinalSummary);
    } catch (error) {
      console.error("Error starting the game:", error);
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
    <main className="flex min-h-screen flex-col items-center px-4 sm:px-8 md:px-16 lg:px-40 py-10">
      <h1 className="text-3xl">King of the Ether 👑</h1>
      <a
        href="https://github.com/phklive/king"
        className="hover:text-blue-500 underline"
      >
        github
      </a>
      <p className="my-10 text-center text-lg font-bold">
        Welcome to "King of the Ether 👑" a real-time game play-out simulation.
        Under the hood a Rust backend spins up a{" "}
        <a
          href="https://github.com/bluealloy/revm"
          className="hover:text-blue-500 underline"
        >
          Revm
        </a>{" "}
        instance deploys the game{" "}
        <a
          href="https://github.com/phklive/King/blob/main/backend/static/king.sol"
          className="hover:text-blue-500 underline"
        >
          smart contract
        </a>{" "}
        within, creates agents with different characteristics / strategies and
        executes the simulation until a new king is crowned 👑
      </p>
      <div className="flex flex-wrap justify-center gap-10">
        {players.map((player, index) => (
          <PlayerCard
            key={index}
            {...player}
            winner={king == player.name}
            value={playerValues[player.name] || 0}
            onChange={handlePlayerValueChange}
          />
        ))}
      </div>
      <button
        onClick={handleSubmit}
        disabled={tooFewPlayers() || isSubmitting}
        className={`my-10 font-bold py-2 px-4 rounded w-1/4 h-16 ${
          tooFewPlayers() || isSubmitting
            ? "bg-gray-500 cursor-not-allowed"
            : "bg-black hover:bg-blue-700"
        }`}
      >
        {isSubmitting ? "Loading..." : "Start"}
        <p>(2 agents minimum)</p>
      </button>
      {summary && <Summary data={summary} />}
    </main>
  );
}

// <Summary text={summary} />
