use heapless::LinearMap;

use crate::state::State;

/// Governor
///
/// Manages all state transitions
#[derive(Debug)]
pub struct Governor<const N: usize> {
    states: LinearMap<u8, State, N>,
    current_state: Option<State>,
    previous_state: Option<State>,
}

impl<const N: usize> Governor<N> {
    /// Returns new governor instance
    ///
    ///```
    /// # use state_governor::Governor;
    /// let governor = Governor::<3>::new(); // Three indicates the maximum amount of states that this governor can handle.
    ///```
    pub const fn new() -> Self {
        Self {
            states: LinearMap::new(),
            current_state: Some(State::unknown()),
            previous_state: Some(State::unknown()),
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
            self.previous_state = self.current_state;
            if let Some(s) = self.states.get(&state_id) {
                self.current_state = Some(*s);
            }
            true
        } else {
            false
        }
    }

    /// Returns current state information
    ///```
    /// # use state_governor::Governor;
    /// # use state_governor::state::State;
    /// # static mut GOVERNOR: Governor<3> = Governor::<3>::new();
    /// # unsafe {
    /// GOVERNOR.add_state(State::new(0x1, "NEW"));
    /// let mut result = GOVERNOR.change_state_to(0x1);
    /// let s = GOVERNOR.get_current_state();
    /// assert_eq!(s.id(), 0x1);
    /// # }
    ///```
    pub fn get_current_state(&mut self) -> State {
        if let Some(s) = self.current_state {
            s
        } else {
            State::unknown()
        }
    }
}
