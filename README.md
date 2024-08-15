# Blockchain-from-scratch
Basic fundamentals of blockchain in Rust.  
Special thanks to [@JoshOrndorff](https://github.com/JoshOrndorff) for creating this tutorial which clearly explains the basic fundamentals of a blockchain.
## This tutorial explains: 
-  How blockchain is simply just a state machine which has a current state that transits into a next state when some action has been performed on it.
- How a block, which consists of a header and and a lot of extrinsics(outside actions), gets executed.

## Table of Contents
### Chapter 1: State Machines
We formalize the notion of a state machine and implement several examples. We do not yet discuss the blockchain data structure or consensus. Examples range from simple toys for learning purposes, to realistic multi-user state machines common in real-world blockchain systems.

- Part 1 - Switch-based state machines - Two dead simple state machines to learn the basics. To run the tests for this chapter, use _cargo test sm_1_.
- Part 2 - Laundry Machine - A toy state machine modeling the lifecycle of clean and dirty laundry. To run the tests for this chapter, use _cargo test sm_2_.
- Part 3 -  Automated Teller Machine - A semi-realistic, but significantly simplified state machine modelling a common ATM. To run the tests for this chapter, use _cargo test sm_3_.
- Part 4 - Accounted Currency - A realistic state machine used as the foundation for many cryptocurrencies such as Ethereum and Polkadot. To run the tests for this chapter, use _cargo test sm_4_.
- Part 5 - Digital Cash - A realistic state machine used as the foundation for many cryptocurrencies such as Monero, Dogecoin, and Litecoin. To run the tests for this chapter, use _cargo test sm_5_.

### Chapter 2: Blockchain
We introduce the blockchain data structure and scaffold it from a simple hash-linked list to a proper blockchain with the Body distinct from the Header, and a consensus digest included. This is the important chapter of the book.

- Part 1 - Header Chain - A minimal hash-linked list with no real state or execution logic. To run the tests for this chapter, use _cargo test bc_1_. 
- Part 2 - Extrinsics and State - We extend our chain to track state and introduce a simple notion of extrinsics. To run the tests for this chapter, use _cargo test bc_2_.
- Part 3 - Consensus - We introduce a basic notion of consensus using proof of work as our first example. To run the tests for this chapter, use _cargo test bc_3_.
- Part 4 - Batched Extrinsics - We separate the block body out of our header, and show that there are multiple extrinsics in a single block. To run the tests for this chapter, use _cargo test bc_4_.
- Part 5 - Fork Choice - We introduce the notion of a fork choice rule and the idea that consumers of the blockchain data structure must decide which of multiple chains is real _for them_. To run the tests for this chapter, use _cargo test bc_5_.
- Part 6 - Rich state - We show that in real-world blockchains the state is not stored directly in the blocks and must be tracked separately. We also introduce the concept of genesis state. To run the tests for this chapter, use _cargo test bc_6_.