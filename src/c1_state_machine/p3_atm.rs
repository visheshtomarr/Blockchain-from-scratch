//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use std::ptr::hash;

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
        let mut new_state = starting_state.clone();
        
        match transition {
            Action::SwipeCard(pin_hash) => {
                match starting_state.expected_pin_hash {
                    Authentication::Waiting => {
                        new_state.expected_pin_hash = Authentication::Authenticating(*pin_hash) ; 
                    }
                    // Ignore "SwipeCard" action if not in Waiting state.
                    _ => {} 
                }
            },
            Action::PressKey(key) => {
                match starting_state.expected_pin_hash {
                    // Ignore key presses if waiting for card swipe.
                    Authentication::Waiting => {},
                    Authentication::Authenticating(expected_pin_hash) => {
                        if *key == Key::Enter {
                            // Check if entered pin's hash is equal to the expected pin hash.
                            let entered_pin_hash = crate::hash(&new_state.keystroke_register) ;
                            if entered_pin_hash == expected_pin_hash {
                                new_state.expected_pin_hash = Authentication::Authenticated ;
                            }
                            else {
                                new_state.expected_pin_hash = Authentication::Waiting ;
                            }
                            new_state.keystroke_register.clear() ;
                        }
                        else {
                            new_state.keystroke_register.push(key.clone()) ;
                        }
                    }
                    Authentication::Authenticated => {
                        if *key == Key::Enter {
                            let amount_to_withdraw = new_state.keystroke_register.iter()
                            .filter_map(|k| match k {
                                Key::One => Some(1),
                                Key::Two => Some(2),
                                Key::Three => Some(3),
                                Key::Four => Some(4),
                                _ => None,
                            }).fold(0, |acc, digit| acc * 10 + digit as u64) ;

                            if amount_to_withdraw <= new_state.cash_inside {
                                new_state.cash_inside -= amount_to_withdraw ;
                            }

                            new_state.expected_pin_hash = Authentication::Waiting ;
                            new_state.keystroke_register.clear() ;
                        }
                        else {
                            new_state.keystroke_register.push(key.clone()) ;
                        }
                    }
                }
            },
        }
        new_state
    }
}

#[cfg(test)]
#[test]
fn sm_3_simple_swipe_card() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    } ;
    let end = Atm::next_state(&start, &Action::SwipeCard(1234)) ;
    let expected =Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: Vec::new(), 
    } ;

    assert_eq!(end, expected) ;
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: Vec::new(),
    } ;
    let end = Atm::next_state(&start, &Action::SwipeCard(1234)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: Vec::new(), 
    } ; 

    assert_eq!(end, expected) ;

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    } ;
    let end = Atm::next_state(&start, &Action::SwipeCard(1234)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    } ;

    assert_eq!(end, expected) ;
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    } ;
    let end = Atm::next_state(&start, &Action::PressKey(Key::One)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    } ;

    assert_eq!(end, expected) ;
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: Vec::new(),
    } ;
    let end = Atm::next_state(&start, &Action::PressKey(Key::One)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: vec![Key::One],
    } ;

    assert_eq!(end, expected) ;

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: vec![Key::One],
    } ;
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two)) ;
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Two],
    } ;

    assert_eq!(end1, expected1) ;
}

#[test]
fn sm_3_enter_wrong_pin() {
    // Create hash of pin.
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four] ;
    let pin_hash = crate::hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(pin_hash),
        keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
    } ;
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    } ;

    assert_eq!(end, expected) ;
}

#[test]
fn sm_3_enter_correct_pin() {
    // Create a hash of pin.
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four] ;
    let pin_hash = crate::hash(&pin) ;

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    } ;
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: Vec::new(),
    } ;

    assert_eq!(end, expected) ;
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: Vec::new(),
    } ;
    let end = Atm::next_state(&start, &Action::PressKey(Key::One)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: vec![Key::One],
    } ;

    assert_eq!(end, expected) ;

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: vec![Key::One],
    } ;
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four)) ;
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    } ;

    assert_eq!(end1, expected1) ;
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    } ;
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter)) ;
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    } ;

    assert_eq!(end, expected) ;
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 9,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_end_to_end_atm_withdraw() {
    let start1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    } ;
    
    // Create hash of pin.
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four] ;
    let pin_hash = crate::hash(&pin) ;

    let end1 = Atm::next_state(&start1, &Action::SwipeCard(pin_hash)) ;
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(pin_hash),
        keystroke_register: Vec::new(),
    } ;

    assert_eq!(end1, expected1) ;

    let start2 = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    } ;
    let end2 = Atm::next_state(&start2, &Action::PressKey(Key::Enter)) ;
    let expected2 = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: Vec::new(),
    } ;

    assert_eq!(end2, expected2) ;

    // Enter amount to withdraw
    let start3 = Atm {
        cash_inside: 10,
        expected_pin_hash: Authentication::Authenticated,
        keystroke_register: vec![Key::Four],
    } ;
    let end3 = Atm::next_state(&start3, &Action::PressKey(Key::Enter)) ;
    let expected3 = Atm {
        cash_inside: 6,
        expected_pin_hash: Authentication::Waiting,
        keystroke_register: Vec::new(),
    } ;

    assert_eq!(end3, expected3) ;
}