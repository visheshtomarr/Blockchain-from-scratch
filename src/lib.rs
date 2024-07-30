use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher} ;

mod c1_state_machine;

#[allow(dead_code)]
/// Simple helper function to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}