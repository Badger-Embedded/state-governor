use heapless::LinearMap;

use crate::state::State;

type TransitionFunc = fn(Option<State>, Option<State>) -> bool;

/// Governor
///
/// Manages all state transitions
#[derive(Debug)]
pub struct Governor<const N: usize> {
    pub states: LinearMap<u8, State, N>,
    pub current_state: Option<State>,
    pub previous_state: Option<State>,
    pub transition_function: Option<TransitionFunc>,
}

impl<const N: usize> Governor<N> {
    /// Returns new governor instance
    ///
    ///```
    /// # use state_governor::Governor;
    /// let governor = Governor::<3>::new(); // Three indicates the maximum amount of states that this governor can handle.
    ///```
    pub fn new() -> Self {
        Self {
            states: LinearMap::new(),
            current_state: Some(State::unknown()),
            previous_state: Some(State::unknown()),
            transition_function: None,
        }
    }

    /// Adds new state to governor.
    /// Returns true if successful otherwise returns false
    ///```
    /// # use state_governor::Governor;
    /// # use state_governor::state::State;
    /// let mut governor = Governor::<3>::new(); // Three indicates the maximum amount of states that this governor can handle.
    /// governor.add_state(State::new(0x1, "NEW"));
    ///```
    pub fn add_state(&mut self, state: State) -> bool {
        if !self.states.contains_key(&state.id) {
            self.states.insert(state.id, state).unwrap();
            true
        } else {
            false
        }
    }

    /// Changes current state to desired one.
    /// Returns true if successful otherwise returns false
    ///```
    /// # use state_governor::Governor;
    /// # use state_governor::state::State;
    /// # {
    /// let mut governor = Governor::<3>::new(); // Three indicates the maximum amount of states that this governor can handle.
    /// governor.add_state(State::new(0x1, "NEW"));
    /// let mut result = governor.change_state_to(0x1);
    /// assert_eq!(result, true);
    /// # }
    /// # {
    /// # let mut governor = Governor::<3>::new(); // Three indicates the maximum amount of states that this governor can handle.
    /// # governor.add_state(State::new(0x1, "NEW"));
    /// # let mut result = true;
    /// result = governor.change_state_to(0x2);
    /// assert_eq!(result, false);
    /// # }
    ///```
    pub fn change_state_to(&mut self, state_id: u8) -> bool {
        if self.states.contains_key(&state_id) {
            if let Some(s) = self.states.get(&state_id) {
                if let Some(transition_func) = self.transition_function {
                    if transition_func(self.current_state, Some(*s)) {
                        self.previous_state = self.current_state;
                        self.current_state = Some(*s);
                        true
                    } else {
                        false
                    }
                } else {
                    self.previous_state = self.current_state;
                    self.current_state = Some(*s);
                    true
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Returns current state information
    ///```
    /// # use state_governor::Governor;
    /// # use state_governor::state::State;
    /// # let mut GOVERNOR: Governor<3> = Governor::<3>::new();
    /// GOVERNOR.add_state(State::new(0x1, "NEW"));
    /// let mut result = GOVERNOR.change_state_to(0x1);
    /// let s = GOVERNOR.get_current_state();
    /// assert_eq!(s.id(), 0x1);
    ///```
    pub fn get_current_state(&mut self) -> State {
        if let Some(s) = self.current_state {
            s
        } else {
            State::unknown()
        }
    }

    /// Sets state transition function. State transition function will be triggered on every state transition event
    /// automatically.
    ///```
    /// # use state_governor::Governor;
    /// # use state_governor::state::State;
    /// let mut governor = Governor::<3>::new(); // Three indicates the maximum amount of states that this governor can handle.
    /// governor.add_state(State::new(0x1, "NEW"));
    /// governor.add_state(State::new(0x2, "OLD"));
    /// static mut CALLED: bool = false;
    /// let mut result = governor.change_state_to(0x1);
    /// fn on_transition(curr_state: Option<State>, next_state: Option<State>) -> bool {
    ///     assert_eq!(curr_state.unwrap().id(), 0x1, "Check that if current state is expected one.");
    ///     assert_eq!(next_state.unwrap().id(), 0x2, "Check that if next state is expected one.");
    ///     unsafe { CALLED = true; }
    ///     return true;
    /// }
    /// governor.set_state_transition_func(on_transition);
    /// result = governor.change_state_to(0x2);
    /// assert_eq!(result, true);
    /// unsafe { assert_eq!(CALLED, true); }
    ///```
    pub fn set_state_transition_func(&mut self, function: TransitionFunc) {
        self.transition_function = Some(function);
    }
}

impl<const N: usize> Default for Governor<N> {
    fn default() -> Self {
        Self::new()
    }
}
