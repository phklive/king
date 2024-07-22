# King of the Ether 👑

[Live website](https://king-zeta.vercel.app/) (Better on desktop 💻)

_"An Ethereum contract is funded with 1,000 ETH. It costs 1 ETH to call, which is added to the balance. If the contract isn't called for 10 blocks, the last caller gets the entire ETH balance. How might this game unfold and end? Describe your thinking." Paradigm_

_"Let's not only think... Let's simulate it!" Paul-Henry_

Rust based real-time simulation environment for the "game" specified in the question enabling the accurate modeling of different agents and game play-out in the context of the Ethereum Virtual Machine ([Revm]()).

## Structure

The code of this repository is separated in 2 main folders a Rust based [backend](./backend/) and React / Typescript based [frontend](./frontend/):

### Backend

The compiled [King.sol](./backend/static/King.sol) is read, a Revm instance is instantiated on which the smart contract is deployed. An [actix-web]() server awaits requests from the frontend containing certain agents with their number and how many rounds of the simulation must be ran. On successful request from the frontend the simulation is executed n number of times using specified generated agents. Finally a summary of run averages is generated and served back to the frontend for display.

### Frontend

The frontend is a [Next.js](https://nextjs.org/) website capable of representing the different agents, select the number of playing agents and rounds played, display the winner and a comprehensive summary of the overall simulation using graphs.

## Setup

1. Clone the repo:

```shell
git clone https://github.com/phklive/king

```

1. Start the Rust server:

```shell
make start-backend
```

2. Start the Next.js frontend:

```shell
make start-frontend
```

## Notes

### Extensibility

This repo can serve as a framewok for Evm "King of the ether" simulations:

- Simple addition of new agents and characteristics
- Implementation of arbitrarly complex agent strategies
- Flexible number of simulation runs
- Programmable Evm transaction context

In the case where this game would be concretely deployed on a blockchain this repo could be used to model and simulation, different scenarios enabling convergence towards a game optimal strategy.

### Agents, strategies & insights

We could think about other agents and strategies:

- Gaz guzzler
- Validator

if same strategies => time game => winner = one with the most funds
if diff strategies => capital efficiency game => winner = one with the most capital efficient strategy
