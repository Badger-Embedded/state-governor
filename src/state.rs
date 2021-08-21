#[derive(Debug, Clone, Copy)]
pub struct State {
    pub(crate) id: u8,
    pub(crate) name: [char; 32],
}

impl State {
    /// Creates new State.
    ///```
    /// # use state_governor::state::State;
    /// let s = State::new(0x1, "NEW");
    /// assert_eq!(s.id(), 0x1);
    /// assert_eq!(s.name().iter().collect::<String>().trim(), "NEW")
    ///```
    pub fn new(id: u8, name: &str) -> Self {
        let mut name_array: [char; 32] = [' '; 32];

        for (i, c) in name.chars().into_iter().enumerate() {
            name_array[i] = c;
        }
        Self {
            id,
            name: name_array,
        }
    }

    /// Creates new State with id 0xFF and name 'UNKNOWN'.
    ///```
    /// # use state_governor::state::State;
    /// let s_unknown = State::unknown();
    /// assert_eq!(s_unknown.id(), 0xFF);
    /// assert_eq!(s_unknown.name().iter().collect::<String>().trim(), "UNKNOWN")
    ///```
    pub const fn unknown() -> Self {
        Self {
            id: 0xFF,
            name: [
                'U', 'N', 'K', 'N', 'O', 'W', 'N', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        }
    }

    /// Returns id of state.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns name of state.
    pub fn name(&self) -> &[char; 32] {
        &self.name
    }
}

impl PartialEq for State {
    /// Equality of [State]s' are decided according to their ids and names.
    ///```
    /// # use state_governor::state::State;
    /// # use state_governor::create_states;
    /// # create_states!(RUN, IDLE, FINISH);
    /// let s1 = State::from(StateEnum::RUN);
    /// assert_eq!(s1.id(), 0);
    /// let s2 = State::new(0, "RUN");
    /// let s3 = State::new(0, "TEST");
    /// assert_eq!(s1, s2);
    /// assert_ne!(s1, s3);
    /// assert_eq!(s1.name(), s2.name());
    ///```
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[macro_export]
macro_rules! create_states {
    ( $($name:ident),* ) => {
        #[derive(Debug, Clone, Copy)]
        pub enum StateEnum {
            $($name),*
        }

        impl StateEnum {
            fn name(&self) -> &'static str {
                match self {
                    $(StateEnum::$name => stringify!($name)),*
                }
            }
        }

        impl core::convert::TryFrom<u8> for StateEnum {
            type Error = ();

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == StateEnum::$name as u8 => Ok(StateEnum::$name), )*
                    _ => Err(()),
                }
            }
        }

        impl From<StateEnum> for state_governor::state::State {
            #[inline]
            fn from(state_enum: StateEnum) -> Self {
                state_governor::state::State::new(state_enum as u8, state_enum.name())
            }
        }

    };
}
