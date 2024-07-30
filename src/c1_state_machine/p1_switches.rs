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