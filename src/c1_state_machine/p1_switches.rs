//! We begin our hands on exploration of state machines with two very simple examples.
//! In these examples, we use actually switch boards as the state machine. The state is,
//! well, just the state of the switches.

use super::StateMachine;

/// This state machine models a single light switch.
/// The internal state is a bool which represents whether the switch is on or not.
pub struct LightSwitch;

/// We model this simple system as a state machine with a single transition - toggling the switch
/// Because there is only a single kind of transition, we can use a unit struct.
impl StateMachine for LightSwitch {
    type State = bool;
    type Transition = ();

    fn next_state(starting_state: &bool, _t: &()) -> bool {
        !starting_state
    }
}

/// The second state machine models two switches with one weird property.
/// Whenever switch one is turned off, switch two also goes off. 
pub struct WeirdSwitchMachine;

/// The state is now two switches instead of one so we use a struct.
#[derive(PartialEq, Eq, Debug)]
pub struct TwoSwitches {
    first_switch: bool,
    second_switch: bool,
}

/// Now, there are two switches so we need a proper type for transition.
pub enum Toggle {
    FirstSwitch,
    SecondSwitch,
}

/// We model this system as a state machine with two possible transtions.
impl StateMachine for WeirdSwitchMachine {
    type State = TwoSwitches;
    type Transition = Toggle;

    fn next_state(starting_state: &TwoSwitches, transition: &Toggle) -> TwoSwitches {
        match transition {
            Toggle::FirstSwitch => TwoSwitches{
                first_switch: !starting_state.first_switch,
                // If the first switch is turned off, second switch automatically gets off.
                second_switch: if starting_state.first_switch{
                    false
                }
                else {
                    starting_state.second_switch
                }
            },
            Toggle::SecondSwitch => TwoSwitches{
                first_switch: starting_state.first_switch,
                second_switch: !starting_state.second_switch,
            },
        }
    }
}

#[cfg(test)]
#[test]
fn sm_1_light_switch_toggles_off() {
    // The starting state of "LightSwitch" was true, i.e., it was "ON".
    // The next state will be false, i.e., "OFF".
    assert!(!LightSwitch::next_state(&true, &())) ;
}

#[test]
fn sm_1_light_switch_toggles_on() {
    // The starting state of "LightSwitch" was false, i.e., it was "OFF".
    // The next state will be true, i.e., "ON".
    assert!(LightSwitch::next_state(&false, &())) ;
}

#[test]
fn sm_1_two_switches_first_goes_on() {
    let state = TwoSwitches {
        first_switch: false,
        second_switch: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoSwitches {
            first_switch: true,
            second_switch: false,
        }
    );
}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_on() {
    // This is the special case. We have to make sure the second one goes off with it.
    let state = TwoSwitches {
        first_switch: true,
        second_switch: true,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoSwitches {
            first_switch: false,
            second_switch: false,
        }
    );
}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_off() {
    // This is adjacent to the special case. We have to make sure the second one stays off.
    let state = TwoSwitches {
        first_switch: true,
        second_switch: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoSwitches {
            first_switch: false,
            second_switch: false,
        }
    );
}

#[test]
fn sm_1_two_switches_second_goes_on() {
    let state = TwoSwitches {
        first_switch: false,
        second_switch: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::SecondSwitch),
        TwoSwitches {
            first_switch: false,
            second_switch: true,
        }
    );
}

#[test]
fn sm_1_two_switches_second_goes_off() {
    let state = TwoSwitches {
        first_switch: true,
        second_switch: true,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::SecondSwitch),
        TwoSwitches {
            first_switch: true,
            second_switch: false,
        }
    );
}