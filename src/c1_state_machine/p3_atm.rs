//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use super::StateMachine;

/// The keys on the ATM keypad.
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM.
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The various states of authentication possible with the ATM.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Authentication {
    /// No session has begun yet. Waiting for the user to swipe the card.
    Waiting,
    /// The user has swiped their card, providing the enclosed pin hash.
    /// Waiting for the user to key in their pin.
    Authenticating(u64),
    /// The user has been authenticated. Waiting for them to key in the amount
    /// of cash they want to withdraw.
    Authenticated,
}

/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, you card is returned 
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM awaits for you to key in the amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Atm {
    /// How much money is in the ATM.
    cash_inside: u64,
    /// The machine's authentication status.
    expected_pin_hash: Authentication,
    /// All the keys have been pressed since the last enter.
    keystroke_register: Vec<Key>
}

impl StateMachine for Atm {
    type State = Self;
    type Transition = Action;

    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State {
        todo!()
    }
}