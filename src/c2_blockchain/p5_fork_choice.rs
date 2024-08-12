//! Forks in blockchain represent alternative histories of the system.
//! When forks arise in the blockchain, users need a way to decide which chain 
//! they will consider best, for now. This is know as the "fork choice rule".
//! There are several meaningful notions of "best", so we introduce a trait 
//! that allow multiple implementations.
//! 
//! Since we have nothing to add to the Block or Header data structures in this lesson,
//! we will import them from the previous lesson.

use super::p4_batched_extrinsics::{Block, Header} ;
use crate::hash ;
use rand::Rng ;

const THRESHOLD: u64 = u64::max_value() / 100 ;

/// Judge which blockchain is "best" when there are multiple candidates. There are several
/// meaningful notions of "best" which is why this is a trait instead of just a
/// method.
pub trait ForkChoice {
    /// Compare two chains, and return the "best" one.
    ///
    /// The chains are not assumed to start from the same genesis block, or even a
    /// genesis block at all. This makes it possible to compare entirely disjoint
    /// histories. It also makes it possible to compare _only_ the divergent part
    /// of sibling chains back to the last common ancestor.
    ///
    /// The chains are assumed to be valid, so it is up to the caller to check
    /// validity first if they are unsure.
    fn first_chain_is_better(chain_1: &[Header], chain_2: &[Header]) -> bool;

    /// Compare many chains and return the best one.
    ///
    /// It is always possible to compare several chains if you are able to compare
    /// two chains. Therefore this method has a provided implementation. However,
    /// it may be much more performant to write a fork-choice-specific implementation.
    fn best_chain<'a>(candidate_chains: &[&'a [Header]]) -> &'a [Header] ;
}

/// The "best" chain is simply the longest chain.
pub struct LongestChainRule ;

impl ForkChoice for LongestChainRule {
    fn first_chain_is_better(chain_1: &[Header], chain_2: &[Header]) -> bool {
        let mut is_better = true ;
        if chain_1.len() < chain_2.len() {
            is_better &= false ;
        }
        is_better
    }

    fn best_chain<'a>(candidate_chains: &[&'a [Header]]) -> &'a [Header] {
        let mut chain_iter = candidate_chains.iter() ;
        let mut best_chain = chain_iter.next().unwrap() ;

        while let Some(next_chain) = chain_iter.next() {
            if next_chain.len() > best_chain.len() {
                best_chain = next_chain
            }
        }
        best_chain
    }
}

/// The "best" chain is the one with the most accumulated work.
/// 
/// In Proof of Work chains, each block contains a certain amount of "work".
/// Roughly speaking, the lower the block's hash is, the more work it contains,
/// because finding a block with a low hash requires, on average, trying more 
/// nonces. Modeling the amount of work required to achieve a particular hash
/// is out of scope for this exercise, so we will use the not-really-right-but
/// conceptually-good-enough formula `work = THRESHOLD - block_hash`.
pub struct HeaviestChainRule ;

/// Generating a random nonce.
fn generate_nonce() -> u64 {
    let mut rng = rand::thread_rng() ;
    return rng.gen::<u32>() as u64;
}

/// Creating a valid header according to Proof of Work.
fn mine_consensus_digest(header: &mut Header, threshold: u64) {
    let mut valid_header = header.clone() ;
    loop {
        let nonce = generate_nonce() ;
        valid_header.consensus_digest = nonce ;
        if hash(&valid_header) < threshold {
            header.consensus_digest = nonce ;
            break;
        }
    }
}

/// Mutates a block (and its embedded header) to contain more PoW difficulty.
/// This will be useful for exploring the heaviest chain rule. The expected
/// usage is that you create a block using the normal `Block.child()` method
/// and then pass the block to this helper for additional mining.
fn mine_extra_hard(block: &mut Block, threshold: u64) {
    mine_consensus_digest(&mut block.header, threshold)
}

impl HeaviestChainRule {
    /// Work done on individual chains.
    fn get_work(chain: &[Header]) -> i64 {
        let mut work = 0 ;
        chain.iter().for_each(|header| {
            work = (work as i64).saturating_add((THRESHOLD - hash(header)) as i64) ;
        }) ;
        work
    }
}

impl ForkChoice for HeaviestChainRule {
    fn first_chain_is_better(chain_1: &[Header], chain_2: &[Header]) -> bool {
        let mut is_better = true ;
        if HeaviestChainRule::get_work(chain_1) < HeaviestChainRule::get_work(chain_2) {
            is_better &= false ;
        }
        is_better
    }

    fn best_chain<'a>(candidate_chains: &[&'a [Header]]) -> &'a [Header] {
        let mut chain_iter = candidate_chains.iter() ;
        let mut best_chain = chain_iter.next().unwrap() ;

        while let Some(next_chain) = chain_iter.next() {
            if HeaviestChainRule::get_work(next_chain) > HeaviestChainRule::get_work(best_chain) {
                best_chain = next_chain ;
            } 
        }
        best_chain
    }
}

/// The best chain is the one with the most blocks that have even hashes.
///
/// This exact rule is a bit contrived, but it does model a family of fork choice rules
/// that are useful in the real world. We just can't code them here because we haven't
/// implemented Proof of Authority yet. Consider the following real world examples
/// that have very similar implementations.
///
/// 1. Secondary authors. In each round there is one author who is supposed to author.
///    If that author fails to create a block, there is a secondary author who may do so.
///    The best chain is the one with the most primary-authored blocks.
///
/// 2. Interleaved Pow/PoA. In each round there is one author who is allowed to author.
///    Anyone else is allowed to mine a PoW-style block. The best chain is the one with
///    the most PoA blocks, and ties are broken by the most accumulated work.
pub struct MostBlocksWithEvenHash ;

impl ForkChoice for MostBlocksWithEvenHash {
    fn first_chain_is_better(chain_1: &[Header], chain_2: &[Header]) -> bool {
        todo!("Sixth")
    }

    fn best_chain<'a>(candidate_chains: &[&'a [Header]]) -> &'a [Header] {
        todo!("Seventh")
    }
}

/// Build and return two different chains with a common prefix.
/// They should have the same genesis header. Both chains should be valid.
/// The first chain should be longer (have more blocks), but the second
/// chain should have more accumulated work.
///
/// Return your solutions as three vectors:
/// 1. The common prefix including genesis
/// 2. The suffix chain which is longer (non-overlapping with the common prefix)
/// 3. The suffix chain with more work (non-overlapping with the common prefix)
fn create_fork_one_side_longer_other_side_heavier() -> (Vec<Header>, Vec<Header>, Vec<Header>) {
    todo!("Eighth")
}