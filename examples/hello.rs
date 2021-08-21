use core::convert::TryInto;

use state_governor::{create_states, state::State, Governor};

create_states! {IDLE, RUN, FINISH}

static mut GOVERNOR: Governor<3> = Governor::<3>::new();

fn state_changer() {
    unsafe {
        match GOVERNOR.get_current_state().id().try_into() {
            Ok(StateEnum::IDLE) => {
                GOVERNOR.change_state_to(StateEnum::RUN as u8);
            }
            Ok(StateEnum::RUN) => {
                GOVERNOR.change_state_to(StateEnum::FINISH as u8);
            }
            Ok(StateEnum::FINISH) => {}
            Err(_) => {}
        }
    }
}

fn state_handler() {
    unsafe {
        match GOVERNOR.get_current_state().id().try_into() {
            Ok(StateEnum::RUN) => {
                println!(
                    "State handler: {}",
                    GOVERNOR
                        .get_current_state()
                        .name()
                        .iter()
                        .collect::<String>()
                        .trim()
                )
            }
            Ok(StateEnum::IDLE) => {
                println!(
                    "State handler: {}",
                    GOVERNOR
                        .get_current_state()
                        .name()
                        .iter()
                        .collect::<String>()
                        .trim()
                )
            }
            Ok(StateEnum::FINISH) => {
                println!(
                    "State handler: {}",
                    GOVERNOR
                        .get_current_state()
                        .name()
                        .iter()
                        .collect::<String>()
                        .trim()
                )
            }
            _ => {}
        }
    }
}

fn main() {
    unsafe {
        GOVERNOR.add_state(State::from(StateEnum::RUN));
        GOVERNOR.add_state(State::from(StateEnum::IDLE));
        GOVERNOR.add_state(State::from(StateEnum::FINISH));
        GOVERNOR.change_state_to(StateEnum::IDLE as u8);
        loop {
            state_handler();
            if GOVERNOR.get_current_state().id() == StateEnum::FINISH as u8 {
                break;
            }
            state_changer();
        }
    }
}
