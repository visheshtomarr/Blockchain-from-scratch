//! We want to make the simplest possible chain to begin with. Just a hash-linked data structure.
//! We learned from the lecture that it is actually the headers that are hash linked, so let's
//! start with that.

use crate::hash;

// We will use Rust's built-in hashing where the output type is u64. I'll make an alias
// so that the code is slightly more readable.
type Hash = u64;

/// The most basic blockchain header possible. We learned its basic structure from lecture.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    // We know from the lecture that we will probably need these, we don't need them yet.
    extrinsics_root: (),
    state_root: (),
    consensus_digest: (),
}

// Here are the methods for creating a new header and verifying headers.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis() -> Self {
        Self {
            parent: 0,
            height: 0,
            extrinsics_root: (),
            state_root: (),
            consensus_digest: (),
        }
    }

    /// Create a return a new valid child header.
    fn child(&self) -> Self {
        Self {
            parent: hash(self),
            height: self.height + 1,
            extrinsics_root: (),
            state_root: (),
            consensus_digest: (),
        }
    }

    /// Verfiy that all the given headers form a valid chain from this header to the tip.
    /// An "entire" chain can be verified by calling this method on a genesis header.
    /// This method may assume that the block on which it is called is valid, but it
    /// must verify all the blocks in the slice.
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut curr_hash = hash(self);
        let mut curr_height = self.height;
        let mut chain_iter = chain.iter();
        let mut is_verified = true;

        while let Some(header) = chain_iter.next() {
            if curr_height.saturating_add(1) != header.height {
                return false;
            }
            is_verified &= curr_hash == header.parent;
            curr_hash = hash(header);
            curr_height = header.height;
        }
        is_verified
    }
}

// And finally a few functions to use the code we just

/// Build and return a chain with exactly five blocks including the genesis block.
fn build_valid_chain_length_5() -> Vec<Header> {
    let g = Header::genesis();
    let mut chain = Vec::new();

    let mut prev_block = g;
    let mut next_block;

    for _ in 0..5 {
        next_block = prev_block.child();
        chain.push(prev_block);
        prev_block = next_block;
    }
    chain
}

/// Build and return a chain with at least three headers.
/// The chain should start with a proper genesis header,
/// but the entire chain should NOT be valid.
fn build_an_invalid_chain() -> Vec<Header> {
    let g = Header::genesis();
    let b1 = g.child();
    let _b2 = b1.child();
    let b2_prime = g.child();

    vec![g, b1, b2_prime]
}

#[cfg(test)]
#[test]
fn bc_1_genesis_block_parent() {
    let g = Header::genesis();
    assert_eq!(g.parent, 0);
}

#[test]
fn bc_1_genesis_block_height() {
    let g = Header::genesis();
    assert_eq!(g.height, 0);
}

#[test]
fn bc_1_child_block_parent() {
    let g = Header::genesis();
    let b1 = g.child();
    assert_eq!(b1.parent, hash(&g));
}

#[test]
fn bc_1_child_block_height() {
    let g = Header::genesis();
    let b1 = g.child();
    assert_eq!(b1.height, 1);
}

#[test]
fn bc_1_verify_genesis_only() {
    let g = Header::genesis();
    assert!(g.verify_sub_chain(&[]));
}

#[test]
fn bc_1_verify_three_blocks() {
    let g = Header::genesis();
    let b1 = g.child();
    let b2 = b1.child();

    assert!(g.verify_sub_chain(&[b1, b2]));
}

#[test]
fn bc_1_cant_verify_invalid_parent() {
    let g = Header::genesis();
    let mut b1 = g.child();
    b1.parent = 5;

    assert!(!g.verify_sub_chain(&[b1]));
}

#[test]
fn bc_1_cant_verify_invalid_height() {
    let g = Header::genesis();
    let mut b1 = g.child();
    b1.height = 5;

    assert!(!g.verify_sub_chain(&[b1]));
}

#[test]
fn bc_1_verify_chain_length_five() {
    let chain = build_valid_chain_length_5();
    assert!(chain[0].verify_sub_chain(&chain[1..]));
}

#[test]
fn bc_1_invalid_chain_is_really_invalid() {
    let invalid_chain = build_an_invalid_chain();
    assert!(!invalid_chain[0].verify_sub_chain(&invalid_chain[1..]));
}
