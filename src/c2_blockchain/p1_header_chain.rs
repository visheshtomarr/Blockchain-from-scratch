//! We want to make the simplest possible chain to begin with. Just a hash-linked data structure.
//! We learned from the lecture that it is actually the headers that are hash linked, so let's 
//! start with that.

use crate::hash ;

// We will use Rust's built-in hashing where the output type is u64. I'll make an alias
// so that the code is slightly more readable.
type Hash = u64 ;

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
    pub fn verify_child(&self, chain: &[Header]) -> bool {
        let mut curr_hash = hash(self) ;
        let mut curr_height = self.height ;
        let mut chain_iter = chain.iter() ;
        let mut is_verified = true ;

        while let Some(header) = chain_iter.next() {
            if curr_height.saturating_add(1) != header.height {
                return false;
            }
            is_verified &= curr_hash == header.parent ;
            curr_hash = hash(header) ;
            curr_height = header.height ;
        }
        is_verified
    }

    // And finally a few functions to use the code we just

    /// Build and return a chain with exactly five blocks including the genesis block.
    fn build_valid_chain_length_5() -> Vec<Header> {
        let g = Header::genesis() ;
        let mut chain = Vec::new() ;
        
        let mut prev_block = g ;
        let mut next_block ;

        for _ in 0..5 {
            next_block = prev_block.child() ;
            chain.push(prev_block) ;
            prev_block = next_block ;
        }
        chain
    }

    /// Build and return a chain with at least three headers.
    /// The chain should start with a proper genesis header,
    /// but the entire chain should NOT be valid.
    pub fn build_an_invalid_chain() -> Vec<Header> {
        let g = Header::genesis() ;
        let b1 = g.child() ;
        let b2 = b1.child() ;
        let b2_prime = g.child() ;

        vec![g, b1, b2_prime]
    }
}