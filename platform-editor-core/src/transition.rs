use std::time::Instant;

#[derive(Debug, Clone, Copy)]
/// A call to take to start a "transition".
pub enum TransitionCall {
    /// Nothing should happen.
    None,
    /// A transition (with the provided duration in nanoseconds) should start.
    Start(u64),
    /// The current transition should stop.
    Stop,
}

/// A manager to handle a transition and its duration.
#[derive(Debug, Clone, Copy)]
pub struct TransitionManager {
    /// The (currently) last starting `Instant` for a transition to start.
    last_transition_instant: Instant,
    /// The entire duration (in nanoseconds) of the current transition, if any.
    transition_end: Option<u64>
}

impl Default for TransitionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionManager {
    pub fn new() -> Self {
        Self {
            last_transition_instant: Instant::now(),
            transition_end: None
        }
    }

    /// Returns whether this manager is currently performing a transition.
    pub fn is_transitioning(&self) -> bool {
        self.transition_end.is_some()
    }

    fn internal_time(&self) -> u64 {
        (Instant::now() - self.last_transition_instant).as_nanos() as u64
    }

    /// Returns the current time of the transition, wrapped in a `Some` if any.
    /// Otherwise, this returns `None`.
    pub fn time(&self) -> Option<u64> {
        self.transition_end.map(|_| self.internal_time())
    }

    /// Ticks this manager, using the provided `TransitionCall` to start a transition,
    /// stop one, or do nothing.
    pub fn tick(&mut self, call: TransitionCall) {
        match call {
            TransitionCall::None => {}
            TransitionCall::Start(duration) => {
                self.last_transition_instant = Instant::now();
                self.transition_end = Some(duration);
            }
            TransitionCall::Stop => {
                self.last_transition_instant = Instant::now();
                self.transition_end = None;
            }
        }

        if self.transition_end.is_some_and(|e| self.internal_time() > e) {
            // Stop the transition already.
            self.last_transition_instant = Instant::now();
            self.transition_end = None;
        }
    }
}
