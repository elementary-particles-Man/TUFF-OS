use anyhow::Result;
use log::{info, warn, error};

mod state_machine;
mod usb_monitor;
mod fs_manager;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("tuffd (TUFF-OS Daemon) starting...");

    // 1. Initial Check
    // TODO: Verify integrity of self

    // 2. State Transition: INIT -> WAIT_KEY
    let mut state = state_machine::SystemState::new();
    state.transition_to(state_machine::State::WaitKey);

    // 3. Start USB Monitor Loop
    info!("Waiting for TUFF-KEY insertion...");
    let usb_event = usb_monitor::wait_for_key().await?;

    if let Some(key) = usb_event {
        info!("Key detected. Verifying...");
        // TODO: Load Key logic
    }

    Ok(())
}
