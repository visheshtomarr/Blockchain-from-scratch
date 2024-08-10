//! Untill now, each block has contained just a single extrinsic. Really we would prefer to batch them.
//! Now, we stop relying solely on headers, and instead, create complete blocks.

use crate::hash;

// We will use Rust's built-in hashing where the output type is u64. I'll make an alias
// so that the code is slightly more readable.
type Hash = u64;

/// The header no longer contains an extrinsic directly. Rather a vector of extrinsics will be stored in
/// the block body. We are still storing state in the header for now. This will change in an uncoming
/// lesson as well.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    // We now switch from storing an extrinsic directly to storing an extrinsic root.
    // This is basically a concise cryptographic commitment to the complete list of extrinsics.
    // For example a hash or a Merkle root.
    extrinsics_root: Hash,
    state: u64,
    pub consensus_digest: u64,
}

// Methods for creating and verifying headers.
//
// With the extrinsics no longer stored in the header, we can no longer do
// "on-chain" execution with just headers. That means that this code actually
// gets simpler in many ways. All the old execution logic, plus some new batching
// logic moves to block level now.
impl Header {
    /// Returns a new valid genesis header.
    pub fn genesis() -> Self {
        Self {
            parent: Hash::default(),
            height: 0,
            extrinsics_root: Hash::default(),
            state: 0,
            consensus_digest: 0,
        }
    }

    /// Create and return a valid child header.
    /// Without the extrinsics themselves, we cannot calculate the final state,
    /// so that information is passed in.
    pub fn child(&self, extrinsics_root: Hash, state: u64) -> Self {
        Self {
            parent: hash(self),
            height: self.height + 1,
            extrinsics_root,
            state,
            consensus_digest: 0,
        }
    }

    /// Verify a single child header.
    ///
    /// This is a slightly different interface from the previous units. Rather
    /// than verify an entire sub-chain, this function checks a single header.
    /// This is useful because checking the header can now be thought of as a
    /// subtask of checking an entire block. So it doesn't make sense to check
    /// the entire header chain at once if the chain may be invalid at the second block.
    fn verify_child(&self, child: &Header) -> bool {
        let parent = self ;
        let mut is_verified = true ;
        if parent.height.saturating_add(1) != child.height {
            return false;
        }
        is_verified &= hash(parent) == child.parent &&  parent.state == child.state ;
        is_verified
    }

    /// Verify that all the headers form a valid chain from this header to the tip.
    ///
    /// We can now trivially write the old verification function in terms of the new one.
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut prev_header = self ;
        let mut prev_header_height = self.height ;
        let mut chain_iter = chain.iter() ;
        let mut is_verified = true ;
        while let Some(header) = chain_iter.next() {
            if prev_header_height.saturating_add(1) != header.height {
                return false ;
            }
            is_verified &= hash(prev_header) == header.parent &&  prev_header.state == header.state ;
            prev_header = header ;
            prev_header_height = header.height ;
        }
        is_verified
    }
}

/// A complete block is a header and the extrinsics.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Block {
    pub(crate) header: Header,
    pub(crate) body: Vec<u64>,
}

// Methods for creating and verifying blocks.
//
// These methods are analogous to the methods on the headers. All of the
// transaction execution logic is now handled at the block level because
// the transactions are no longer available at the Header level.
impl Block {
    /// Returns a new valid genesis block. By convention, this block has no extrinsics.
    pub fn genesis() -> Self {
        todo!("Fifth")
    }

    /// Create and return a valid child block.
    /// The extrinsics are batched now, so we need to execute each one of them.
    pub fn child(&self, extrinsics: Vec<u64>) -> Self {
        todo!("Sixth")
    }

    /// Verify that all the given blocks form a valid chain from this block to the tip.
    /// We need to verify the headers as well as execute all transactions and check the final state.
    pub fn verify_sub_chain(&self, chain: &[Block]) -> bool {
        todo!("Seventh")
    }
}

/// Create an invalid child block of the given block. Although the child block is invalid,
/// the header should be valid.
///
/// Now that extrinsics are separate from headers, the logic for checking headers does
/// not include actual transaction execution. That means it is possible for a header to be
/// valid, but the block containing that header to be invalid.
fn build_invalid_child_block_with_valid_header(parent: &Header) -> Block {
    todo!("Eighth")
}