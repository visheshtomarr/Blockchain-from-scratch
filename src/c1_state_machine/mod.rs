//! This module is all about modeling phenomena and systems as state machines. We begin with a few simple
//! examples, and then proceed to build bigger and more complex state machines all implementing the same simple interface.

mod p1_switches;

/// A state machine - Generic over the transition type 
pub trait StateMachine {
    /// The States that can be occupied by this machine.
    type State;
    
    /// The transitions that can be made between states.
    type Transition ;

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State ; 
}

/// A set of play users for experimenting with the multi-user state machines.
pub enum User {
    Alice,
    Bob,
    Charlie,
}