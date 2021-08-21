//! # StateGovernor
//!
//! state-governor is a simple state management library focused on embedded systems.
#![no_std]

pub mod governor;
pub mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
