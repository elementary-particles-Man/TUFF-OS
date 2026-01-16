#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Init,
    WaitKey,
    Normal,
    Warn,
    Freeze,
    Recovery,
}

pub struct SystemState {
    current: State,
}

impl SystemState {
    pub fn new() -> Self {
        Self { current: State::Init }
    }

    pub fn transition_to(&mut self, next: State) {
        log::info!("State Transition: {:?} -> {:?}", self.current, next);
        self.current = next;
    }
}
