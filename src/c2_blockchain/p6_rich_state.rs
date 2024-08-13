//! In this lesson, we expand our simple notion of state, and show how the state is typically not stored in the header,
//! Or indeed anywhere in the Block at all.
//! 
//! To facilitate this exercise, consider that we want our blockchain to store not the sum of extrinsics,
//! but also the product. You can also imagine any other calculations the chain may want to track (min, max, median, mean, etc).
//! 
//! As the state data gets large, it is no longer reasonable to store it in the blocks. But if the state isn't in the blocks,
//! then how can we perform the state-related validation checks we previously performed? We use a state root to cryptographically
//! link our header to a complete state.
//! 
//! This notion of state may sound familiar from our previous work on state machines. Indeed this
//! naming coincidence foreshadows a key abstraction that we will make in a coming chapter.

type Hash = u64 ;
use std::io::Chain;

use crate::hash ;
use super::p3_consensus::THRESHOLD ;

/// In this section, we will use sum and product together to be a part of our state. While this is only a doubling of state size,
/// remember that in real world blockchains, the state is often really really large.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct State {
    sum: u64,
    product: u64,
}

/// The header no longer contains the state directly, but rather, it contains a hash of 
/// the complete state. This hash will allow block verifiers to cryptographically confirm
/// that they got the same state as the author without having a complete copy of the
/// author's state.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    extrinsics_root: Hash,
    // Stores a cryptographic commitment, like a Merkle root or a hash to the complete
    // post state.
    state_root: Hash,
    consensus_digest: u64,
}

/// Methods for creating and verifying headers.
///
/// We already moved the execution logic to the block level in the last section.
/// So this code is similar to last time. One key addition we are making is that
/// genesis blocks can have an initital state, or "genesis state" other than the
/// default. So we need to commit the initial state root to the genesis header here.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis(genensis_state_root: Hash) -> Self {
        Self {
            parent: Hash::default(),
            height: 0,
            extrinsics_root: Hash::default(),
            state_root: genensis_state_root,
            consensus_digest: 0, 
        }
    }

    /// Create and return a valid child header.
    /// 
    /// The state root is passed in similarly to how the complete state
    /// was in the previous section.
    fn child(&self, extrinsics_root: Hash, state_root: Hash) -> Self {
        Self {
            parent: hash(self),
            height: self.height + 1,
            extrinsics_root,
            state_root,
            consensus_digest: 0,
        }
    }

    /// Verify a single child header.
    fn verify_child(&self, child: &Header) -> bool {
        let mut is_verified = true ;
        let parent_header = self ;
        is_verified &= hash(parent_header) == child.parent && parent_header.height.saturating_add(1) == child.height ;
        is_verified    
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut parent_header = self ;
        let mut chain_iter = chain.iter() ;
        let mut is_verified = true ;

        while let Some(child_header) = chain_iter.next() {
            is_verified &= parent_header.verify_child(child_header) ;
            parent_header = child_header ;
        }
        is_verified
    }
}

/// A complete block is a header and the extrinsics.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Block {
    pub(crate) header: Header,
    pub(crate) body: Vec<u64>,
}

/// Methods for creating and verifying blocks.
/// 
/// We no longer have access to a state simply by having access to a block.
/// Therefore, we need a pre-state explicitly passed for these block methods.
/// In a real blockchain network, the client is typically responsible for
/// storing some likely-to-be-needed states and have them ready for use in
/// such operations.
///
/// These methods also differ from last time because you will need to
/// calculate state roots to pass to the header-level methods.
impl Block {
    /// Returns a valid genesis block. By convention this block has no extrinsics.
    pub fn genesis(genesis_state: &State) -> Self {
        Self {
            header: Header::genesis(hash(genesis_state)),
            body: Vec::new(),
        }
    }

    /// Create and return a valid child block.
    pub fn child(&self, pre_state: &State, extrinsics: Vec<u64>) -> Self {
        todo!("Sixth")
    }

    /// Verify that all the given blocks form a valid chain from this block to the tip.
    /// 
    /// This time we need to validate the initial block itself by confirming that we
    /// have been given a valid pre-state. And we still need to verify the headers,
    /// execute all transactions, and check the final state.
    pub fn verify_sub_chain(&self, pre_state: &State, chain: &[Block]) -> bool {
        todo!("Seventh")
    }
}

/// Create an invalid child block of the given block. The returned block should have an
/// incorrect state root. Although the child block is invalid, the header should be valid.
///
/// As we saw in the previous section, the logic for checking headers can no longer
/// not include actual transaction execution, making it possible for invalid blocks
/// to still contain valid headers. There are now two ways to accomplish this.
/// 1. Block includes wrong extrinsics that do not match extrinsics root in header (from last time)
/// 2. Block contains invalid state root that does not match the correct post state (new this time)
///
/// As before, you do not need the entire parent block to do this. You only need the header.
/// You do, however, now need a pre-state as you have throughout much of this section.
fn build_invalid_child_block_with_valid_header(parent: &Header, pre_state: &State) -> Block {
    todo!("Eighth")
}