use log::{info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Init,
    WaitKey,
    Normal,
    Warn,
    Freeze,
    PendingOnly,
    Shutdown,
}

pub struct SystemState {
    current: State,
}

impl SystemState {
    pub fn new() -> Self {
        Self { current: State::Init }
    }

    pub fn current(&self) -> State {
        self.current
    }

    /// Attempt to transition to a new state.
    /// Returns true if transition allowed, false otherwise.
    pub fn transition_to(&mut self, next: State) -> bool {
        if !self.can_transition(next) {
            warn!("Invalid state transition attempted: {:?} -> {:?}", self.current, next);
            return false;
        }

        info!("State Transition: {:?} -> {:?}", self.current, next);
        self.current = next;
        true
    }

    fn can_transition(&self, next: State) -> bool {
        match (self.current, next) {
            // Init can go to WaitKey (boot flow) or Freeze (error)
            (State::Init, State::WaitKey) => true,
            (State::Init, State::Freeze) => true,

            // WaitKey goes to Normal upon success
            (State::WaitKey, State::Normal) => true,
            (State::WaitKey, State::Freeze) => true,

            // Normal operation
            (State::Normal, State::Warn) => true,
            (State::Normal, State::Freeze) => true,
            (State::Normal, State::PendingOnly) => true,
            (State::Normal, State::Shutdown) => true,

            // Warn can recover or worsen
            (State::Warn, State::Normal) => true,
            (State::Warn, State::Freeze) => true,

            // Freeze is a trap. Only explicit Admin intervention or Shutdown can exit.
            (State::Freeze, State::Normal) => true,
            (State::Freeze, State::Shutdown) => true,

            // PendingOnly (Queue processing without new writes)
            (State::PendingOnly, State::Normal) => true,
            (State::PendingOnly, State::Freeze) => true,

            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{State, SystemState};

    #[test]
    fn init_transitions_only_allow_waitkey_or_freeze() {
        let mut state = SystemState::new();
        assert!(state.transition_to(State::WaitKey));

        let mut state = SystemState::new();
        assert!(state.transition_to(State::Freeze));

        let mut state = SystemState::new();
        assert!(!state.transition_to(State::Normal));
    }

    #[test]
    fn waitkey_to_normal_allowed() {
        let mut state = SystemState::new();
        assert!(state.transition_to(State::WaitKey));
        assert!(state.transition_to(State::Normal));
    }

    #[test]
    fn normal_flow_and_recovery_paths() {
        let mut state = SystemState::new();
        assert!(state.transition_to(State::WaitKey));
        assert!(state.transition_to(State::Normal));
        assert!(state.transition_to(State::Warn));
        assert!(state.transition_to(State::Normal));
        assert!(state.transition_to(State::Freeze));
        assert!(state.transition_to(State::Shutdown));
    }

    #[test]
    fn invalid_transitions_are_rejected() {
        let mut state = SystemState::new();
        assert!(!state.transition_to(State::Shutdown));
        assert!(!state.transition_to(State::PendingOnly));
    }
}
