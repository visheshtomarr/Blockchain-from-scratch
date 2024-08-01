//! The state machines we have written so far model individual devices that are typically used by a
//! single user at a time. State machines can also model multi-user systems. Blockchains
//! strive to provide reliable public infrastructure. And the public is very much multiple users.
//! 
//! In this module and the next, we explore two common techniques at modeling multi-user state
//! machines. In this module, we explore accounts and in the next, we explore UTXOs.
//! 
//! In this module we design a state machine that tracks the currency balances of several users.
//! Each user is associated with an account balance and users are able to send money to other users.

use super::{StateMachine, User} ;
use std::collections::HashMap ;

/// This state machine models a multi-user currency system. It tracks the balance of each user
/// and allows user to send funds to one another.
pub struct AccountedCurrency ;

/// The main balances mapping.
/// 
/// Each entry maps a user id to their corresponding balance.
/// There exists an existential deposit of atleast 1. That is 
/// to say that an account gets removed from the map entirely
/// when its balance falls back to 0.
type Balances = HashMap<User, u64> ;

/// The state transitions that users can make in an accounted currency system.
pub enum AccountingTransaction {
    /// Create some new money for the given minter in the given amount.
    Mint { minter: User, amount: u64},
    /// Destroy some money from the given account in the given amount.
    /// If burn amount exceeds the account balance, burn the entire amount 
    /// and remove the account from the storage.
    Burn { burner: User, amount: u64},
    /// Send some amount from one account to another.
    Transfer {
        sender: User,
        receiver: User,
        amount: u64,
    }
}

impl StateMachine for AccountedCurrency {
    type State = Balances;
    type Transition = AccountingTransaction;

    fn next_state(starting_state: &Balances, transition: &AccountingTransaction) -> Balances {
        todo!()
    }
}