use state_governor::{create_states, governor::Governor, state::State};

create_states! {IDLE, RUN, FINISH}

fn idle_handler(governor: Governor<3>, prev_state: State) {
    println!("{:?} {:?}", governor, prev_state);
}

static mut GOVERNOR: Governor<3> = Governor::<3>::new();

fn main() {
    let mut s = State::from(StateEnum::RUN);

    unsafe {
        GOVERNOR.add_state(s);
        GOVERNOR.change_state_to(StateEnum::RUN as u8);
        println!(
            "{:?}",
            GOVERNOR
                .get_current_state()
                .name()
                .iter()
                .collect::<String>()
                .trim()
        );
    }
}
