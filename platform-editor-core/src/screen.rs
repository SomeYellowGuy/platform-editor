/// Represents a screen in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Title,
    LevelSelect,
    Level,
    Options,
}

use std::time::Instant;

#[derive(Debug, Clone, Copy)]
/// A call to take to start a "transition".
pub enum TransitionCall {
    /// Nothing should happen.
    None,
    /// A transition (with the provided duration in nanoseconds) should start, along with
    /// switching to the provided screen.
    Start(TransitionData),
    /// The current transition should stop.
    Stop,
}

/// A manager to handle screens, transitions, and their durations.
#[derive(Debug, Clone, Copy)]
pub struct ScreenManager {
    /// The current screen of the game.
    pub screen: Screen,
    /// The (currently) last starting `Instant` for a transition to start.
    last_transition_instant: Instant,
    /// The entire duration (in nanoseconds) of the current transition, if any.
    transition_data: Option<TransitionData>,
}

#[derive(Debug, Clone, Copy)]
pub enum TransitionData {
    Entry {
        // The total duration of the transition.
        duration: u64,
        /// The previous screen.
        previous_screen: Screen,
    },
    Exit {
        // The total duration of the transition.
        duration: u64,
        /// The new screen.
        new_screen: Screen,
        /// The time for the entry transition immediately after this one.
        enter_time: u64,
    },
}

impl TransitionData {
    /// Creates data for a new (exit) transition, .
    ///
    /// Both times are in nanoseconds.
    pub const fn new(exit_time: u64, enter_time: u64, new_screen: Screen) -> Self {
        Self::Exit {
            duration: exit_time,
            new_screen,
            enter_time,
        }
    }

    pub fn is_exit(&self) -> bool {
        matches!(self, Self::Exit { .. })
    }

    fn other_screen(&self) -> Screen {
        match self {
            Self::Entry {
                previous_screen, ..
            } => *previous_screen,
            Self::Exit { new_screen, .. } => *new_screen,
        }
    }

    fn time(&self) -> u64 {
        match self {
            Self::Entry { duration, .. } | Self::Exit { duration, .. } => *duration,
        }
    }
}

/// Public transition data for rendering.
#[derive(Debug, Clone, Copy)]
pub struct TransitionStatus {
    /// Returns the current time of the transition, in nanoseconds, wrapped in a `Some` if any.
    /// Otherwise, this returns `None`.
    ///
    /// - For an exit transition, this returns the time elapsed.
    /// - For an entry transition, this returns the time **left**.
    pub time: u64,

    /// The previous or next screen, depending on the type of transition.
    /// - For an exit transition, this returns the *next screen*.
    /// - For an entry transition, this returns the *previous screen*.
    pub screen: Screen,

    /// Returns whether this transition is an exit transition.
    pub is_exit: bool,
}

impl Default for ScreenManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            screen: Screen::Title,
            last_transition_instant: Instant::now(),
            transition_data: None,
        }
    }

    /// Returns whether this manager is currently performing a transition.
    pub fn is_transitioning(&self) -> bool {
        self.transition_data.is_some()
    }

    fn internal_time(&self) -> u64 {
        (Instant::now() - self.last_transition_instant).as_nanos() as u64
    }

    /// Returns the current time of the transition, wrapped in a `Some` if any.
    /// Otherwise, this returns `None`.
    ///
    /// - For an exit transition, this returns the time elapsed.
    /// - For an entry transition, this returns the time **left**.
    pub fn time(&self) -> Option<u64> {
        self.transition_data.map(|data| {
            if data.is_exit() {
                self.internal_time()
            } else {
                data.time().saturating_sub(self.internal_time())
            }
        })
    }

    /// Ticks this manager, using the provided `TransitionCall` to start a transition,
    /// stop one, or do nothing.
    pub fn tick(&mut self, call: TransitionCall) -> Option<(Screen, Screen)> {
        match call {
            TransitionCall::None => {}
            TransitionCall::Start(data) => {
                self.last_transition_instant = Instant::now();
                self.transition_data = Some(data);
            }
            TransitionCall::Stop => {
                self.last_transition_instant = Instant::now();
                self.transition_data = None;
            }
        }

        if let Some(data) = self.transition_data
            && self.internal_time() > data.time()
        {
            if let TransitionData::Exit {
                new_screen,
                enter_time,
                ..
            } = data
            {
                // Exit transition ended: start the enter transition.
                // Change the screen.
                let old_screen = self.screen;
                self.screen = new_screen;
                self.last_transition_instant = Instant::now();

                self.transition_data = Some(TransitionData::Entry {
                    duration: enter_time,
                    previous_screen: old_screen,
                });

                return Some((old_screen, self.screen));
            } else {
                self.transition_data = None;
            }
        }

        None
    }

    pub fn transition_status(&self) -> Option<TransitionStatus> {
        self.transition_data.map(|data| TransitionStatus {
            time: self.time().unwrap(),
            is_exit: data.is_exit(),
            screen: data.other_screen(),
        })
    }
}
