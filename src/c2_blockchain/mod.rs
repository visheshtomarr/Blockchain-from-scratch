//! This module explores the blockchain. A distributed hash-linked tree-like data structure that is used
//! to track the alternative histories of a shared resource. It also explores a simple work-based consensus
//! algorithm to help users decide which history is the canonical one.

mod p1_header_chain;
mod p2_extrinsic_state;
mod p3_consensus;
mod p4_batched_extrinsics;
mod p5_fork_choice;
mod p6_rich_state;