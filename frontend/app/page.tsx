"use client";
import React, { useState, useEffect } from "react";
import PlayerCard from "./components/player";
import { Player } from "./types";
import Summary from "./components/summary";

export default function Home() {
  const [players, setPlayers] = useState<Player[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [playerValues, setPlayerValues] = useState<{ [key: string]: number }>(
    {},
  );
  const [summary, setSummary] = useState("");

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

  const handleSubmit = async () => {
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
      const res = JSON.stringify(result);
      console.log("Game started successfully:", result);
      setSummary(res);
    } catch (error) {
      console.error("Error starting the game:", error);
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
      <a href="https://github.com/phklive/king" className="hover:text-blue-500">
        github
      </a>
      <p className="my-10">Quote about the game;</p>
      <div className="flex flex-wrap justify-center gap-10">
        {players.map((player, index) => (
          <PlayerCard
            key={index}
            {...player}
            value={playerValues[player.name] || 0}
            onChange={handlePlayerValueChange}
          />
        ))}
      </div>
      <p className="my-10">Explanation of the game yatiyatiyata</p>
      <button
        onClick={handleSubmit}
        className="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        Start
      </button>
      <Summary text={summary} />
    </main>
  );
}
