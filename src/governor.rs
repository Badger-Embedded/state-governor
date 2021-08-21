use heapless::LinearMap;

use crate::state::State;

/// Governor
///
/// Manages all state transitions
#[derive(Debug)]
pub struct Governor<'a, const N: usize> {
    states: LinearMap<u8, State, N>,
    current_state: Option<&'a State>,
    previous_state: Option<&'a State>,
}

impl<'a, const N: usize> Governor<'a, N> {
    /// Returns new governer instance
    ///
    ///```
    /// # use state_governor::Governor;
    /// let governor = Governor::<3>::new(); // Three indicates the maximum amount of states that this governor can handle.
    ///```
    pub const fn new() -> Self {
        Self {
            states: LinearMap::new(),
            current_state: None,
            previous_state: None,
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
    pub fn change_state_to(&'a mut self, state_id: u8) -> bool {
        if self.states.contains_key(&state_id) {
            self.previous_state = self.current_state;
            self.current_state = self.states.get(&state_id);
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
    pub fn get_current_state(&mut self) -> &'a State {
        self.current_state.unwrap()
    }
}
