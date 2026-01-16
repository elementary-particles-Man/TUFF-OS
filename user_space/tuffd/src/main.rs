use anyhow::Result;
use log::info;

mod state_machine;
mod usb_monitor;
mod fs_manager;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("tuffd (TUFF-OS Daemon) starting on TF-Core...");
    // 1. Initial State: Wait for Key
    let mut state = state_machine::SystemState::new();
    state.transition_to(state_machine::State::WaitKey);

    // 2. Monitor USB
    info!("Waiting for TUFF-KEY insertion...");
    let _usb_event = usb_monitor::wait_for_key().await?;

    Ok(())
}
