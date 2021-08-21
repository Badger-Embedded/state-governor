use heapless::LinearMap;

use crate::state::State;

#[derive(Debug)]
pub struct Governor<'a, const N: usize> {
    states: LinearMap<u8, State, N>,
    pub current_state: Option<&'a State>,
    pub previous_state: Option<&'a State>,
}

impl<'a, const N: usize> Governor<'a, N> {
    pub const fn new() -> Self {
        Self {
            states: LinearMap::new(),
            current_state: None,
            previous_state: None,
        }
    }

    pub fn add_state(&mut self, state: State) -> bool {
        if !self.states.contains_key(&state.id) {
            self.states.insert(state.id, state).unwrap();
            true
        } else {
            false
        }
    }

    pub fn change_state_to(&'a mut self, state_id: u8) -> bool {
        if self.states.contains_key(&state_id) {
            self.previous_state = self.current_state;
            self.current_state = self.states.get(&state_id);
            true
        } else {
            false
        }
    }

    pub fn get_current_state(&mut self) -> &'a State {
        self.current_state.unwrap()
    }
}
