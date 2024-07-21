# King of the Ether 👑

In the context of the [Paradigm fellowship](https://www.paradigm.xyz/2024/06/paradigm-fellowship-2024) application this is an attempt to answer the following technical question:

_"An Ethereum contract is funded with 1,000 ETH. It costs 1 ETH to call, which is added to the balance. If the contract isn't called for 10 blocks, the last caller gets the entire ETH balance. How might this game unfold and end? Describe your thinking."_

_King of the Ether 👑_ is a Rust based simulation environment for the "game" specified in the question.

The code is separated in 2 folders [frontend](./frontend/) and [backend](./backend/):

## Backend

The compiled [King.sol](./backend/static/King.sol) is read, a Revm instance is instantiated on which the smart contract is deployed. An [actix-web] server awaits requests from the frontend containing certain agents with their number and how many rounds of the simulation must be ran. On successful request from the frontend the simulation is executed n number of times using specified generated agents.

## Frontend

The frontend is a [Next.js](https://nextjs.org/) website capable of representing the different agents, select the number of playing agents and rounds played, display the winner and a comprehensive summary of the overall simulation.

## Framework

This framework enables the simple implementation of new agents with different characteristics and strategies enabling the accurate modeling of the play out of this "game" in the context of the Ethereum virtual machine.

## Conclusion

This sets an extendable boilerplate

### Strategies

if same strategies => time game => winner = one with the most funds
if diff strategies => capital efficiency game => winner = one with the most capital efficient strategy
