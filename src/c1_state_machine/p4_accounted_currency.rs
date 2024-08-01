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
        use AccountingTransaction::* ;

        let mut new_state = starting_state.clone() ;

        match transition {
            Mint { minter, amount } => {
                // If the mint amount is equal to 0, we don't mint anything.
                if *amount == 0 {
                    return new_state;
                }
                let balances = new_state.entry(*minter).or_insert(0) ;
                *balances += amount ;
            }
            Burn { burner, amount} => {
                // If burner is not present in the Balances map, we don't burn anything.
                if !new_state.contains_key(burner) {
                    return new_state;
                }
                // Get old amount of burner.
                let old_amount = *new_state.get(burner).unwrap() ;

                // Calculate new amount for burner.
                let new_amount = old_amount.saturating_sub(*amount);

                // If the new amount results into less than or equal to zero, we remove the user, else,
                // we update the Balances map with new amount.
                if new_amount <= 0 {
                    new_state.remove(burner) ;
                }
                else {
                    new_state.insert(*burner, *amount) ;
                }
            }
            Transfer { sender, receiver, amount} => {
                // If the sender or receiver is unregistered, we don't transfer anything.
                if !new_state.contains_key(sender) || !new_state.contains_key(receiver) {
                    return new_state;
                }

                // Get balance amount of sender.
                let old_amount_of_sender = *new_state.get(sender).unwrap() ;

                // If the amount to be sent is greater than the balance amount of sender, 
                // we don't transfer anyting.
                if old_amount_of_sender < *amount {
                    return new_state;
                } 

                // If the sender and receiver are same user, we don't transfer anything.
                if new_state.get(sender) == new_state.get(receiver) {
                    return new_state;
                }

                // If the receiver does not exist in the Balances map in the starting state, 
                // we insert the receiver with balance amount, else, if the receiver is pre-existing,
                // we get the old balance of receiver and update it.
                if !new_state.contains_key(receiver) {
                    new_state.insert(*receiver, *amount) ;
                    let new_amount_of_sender = old_amount_of_sender.saturating_sub(*amount) ;
                    if new_amount_of_sender <= 0 {
                        new_state.remove(sender) ;
                    }
                    else {
                        new_state.insert(*sender, new_amount_of_sender) ;
                    }
                } else {
                    // Get balance of receiver.
                    let old_amount_of_receiver = *new_state.get(receiver).unwrap() ;

                    // Calculate the updated balance of receiver and sender.
                    let new_amount_of_sender = old_amount_of_sender.saturating_sub(*amount) ;
                    let new_amount_of_receiver = old_amount_of_receiver.saturating_sub(*amount) ;
                    if new_amount_of_sender <= 0 {
                        new_state.remove(sender) ;
                    } else {
                        new_state.insert(*sender, new_amount_of_sender) ;
                    }
                    new_state.insert(*receiver, new_amount_of_receiver) ;
                }
            }
        }
        new_state
    }
}