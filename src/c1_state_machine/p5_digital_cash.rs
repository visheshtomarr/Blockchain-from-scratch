//! In this module we design another multi-user currency system. This one is not based on accounts 
//! but rather, is modelled after a paper cash system. The system tracks individual cash 
//! bills. Each bill has an amount and an owner, and can be spent in its entirety. When 
//! a state transition spends bills, new bills are created in lesser or equal amounts.

use super::{StateMachine, User} ;
use std::collections::{HashMap,HashSet} ;

/// This state machine models a multi-user currency system. It tracks a set of bills 
/// in circulation, and updates the set when money is transferred.
pub struct DigitalCashSystem;

/// A single bill in the digital cash system. Each bill has an owner who is allowed to spent it
/// and an amount that it is worth. It also has a serial number to ensure that each bill
/// is unique.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Bill {
    owner: User,
    amount: u64,
    serial: u64,
}

/// The State of the digital cash system. Primarily, it is just a set of circulating bills,
/// but also a counter for the next serial number.
pub struct State {
    /// The set of currently circulating bills.
    bills: HashSet<Bill>,
    /// The next serial number to use when a bill is created.
    next_serial: u64,
}

impl State {
    // Create a new instance of our State.
    pub fn new() -> Self {
        Self {
            bills: HashSet::new(),
            next_serial: 0,
        }
    }

    // Set serial for the created bill.
    pub fn set_serial(&mut self, serial: u64) {
        self.next_serial = serial ; 
    }

    // Return an instance of next serial number.
    pub fn next_serial(&self) -> u64 {
        self.next_serial
    }

    // Increment serial by 1.
    pub fn increment_serial(&mut self) {
        self.next_serial += 1 ;
    }

    // Add new bill to the Bill's set.
    fn add_bill(&mut self, elem: Bill) {
        self.bills.insert(elem) ;
        self.increment_serial() ; 
    }
}

impl FromIterator<Bill> for State {
    fn from_iter<T: IntoIterator<Item = Bill>>(iter: T) -> Self {
        let mut state = State::new() ;

        for i in iter {
            state.add_bill(i)
        }
        state
    }
}

impl <const N: usize> From<[Bill; N]> for State {
    fn from(value: [Bill; N]) -> Self {
        State::from(value)
    }
}

/// The state transitions that users can make in the digital cash system.
pub enum CashTransaction {
    /// Mint a single new bill owned by the minter.
    Mint { minter: User, amount: u64},
    /// Send some money from some users to other users. The money does not all need to 
    /// come from the same user, and it does not all need to go to the same user.
    /// The total amount received must be less than or equal to the amount spent.
    /// The discrepancy between the amount sent and received is destroyed. Therefore,
    /// no dedicated burn transaction is required.
    Transfer {
        spends: Vec<Bill>,
        receives: Vec<Bill>,
    },
}

/// We model this system as a state machine with two possible transitions.
impl StateMachine for DigitalCashSystem {
    type State = State; 
    type Transition = CashTransaction;

    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State {
        todo!()
    }
}