# King of the Ether 👑

Inspired by Paradigm's question:

> "An Ethereum contract is funded with 1,000 ETH. It costs 1 ETH to call, which is added to the balance. If the contract isn't called for 10 blocks, the last caller gets the entire ETH balance. How might this game unfold and end?"

This project simulates the game in real-time, using a Rust-based environment to model different agents and strategies within the context of the Ethereum Virtual Machine ([Revm](https://github.com/bluealloy/revm)).

## Structure

The repository is divided into two main components:

1. [Backend](./backend/): Rust-based simulation engine
2. [Frontend](./frontend/): React/TypeScript-based user interface

### Backend

- Reads and deploys the compiled [King.sol](./backend/static/king.sol) contract
- Uses Revm to instantiate an EVM environment
- Runs an [actix-web](https://actix.rs/) server to handle frontend requests
- Executes simulations with specified agents and rounds
- Generates and serves summary statistics

### Frontend

- Built with [Next.js](https://nextjs.org/)
- Allows users to select agents, number of players, and simulation rounds
- Displays winner and comprehensive simulation summary using graphs

## Setup

1. Make sure that you have `rust`, `npm` and `next` installed

2. Clone the repository:

```shell
git clone https://github.com/phklive/king

```

3. Start the Rust server:

```shell
make start-backend
```

4. Start the Next.js frontend:

```shell
make start-frontend
```

## Extensibility

This project serves as a framework for EVM "King of the Ether" simulations:

- Easy addition of new agents and characteristics
- Implementation of arbitrarily complex agent strategies
- Flexible number of simulation runs
- Programmable EVM transaction context

For real-world blockchain deployments, this framework could be used to model various scenarios and help determine concrete optimal game strategies.

## Agents, Strategies & Insights

The current implementation features basic agents with simple strategies. However, there is room for improvement in agent complexity and strategy granularity. Here are some potential enhancements:

### Proposed New Agents

1. **Validator**

   - Ability to censor incoming transactions attempting to claim the throne
   - Limited censorship power (e.g., can only censor once every X blocks)
   - Introduces an element of transaction control to the game

2. **Gas Guzzler**

   - Can fill an entire block with their own transactions
   - Limited to one-time use per game (is costly)
   - Adds a dimension of block space competition

### Strategy Refinements

- Develop coalition-forming capabilities among agents
- Create adaptive strategies that evolve based on other agents behaviors

### Game Dynamics Insights

Based on current simulations, two primary game dynamics emerge:

1. **Homogeneous Strategies**

   - When all agents employ similar strategies, the game becomes a war of attrition
   - The agent with the largest capital pool typically emerges victorious
   - Game outcome is largely determined by initial resource distribution

2. **Heterogeneous Strategies**

   - With diverse agent strategies, capital efficiency becomes the key to success
   - Clever tactics can outperform raw financial power
   - The game shifts from a simple endurance test to a complex strategic challenge

By enhancing agent complexity and strategy diversity, we can create a more nuanced and realistic simulation of on-chain game theory dynamics.
