use anyhow::Result;
use log::{info, error};
use tokio::time::{sleep, Duration};

mod state_machine;
mod usb_monitor;
mod fs_manager;

use state_machine::{SystemState, State};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("tuffd (TUFF-OS Daemon) starting...");

    // 1. Initialize State Machine
    let mut state = SystemState::new();

    // 2. Transition to WAIT_KEY (Boot sequence)
    if !state.transition_to(State::WaitKey) {
        error!("Failed to transition to WaitKey state. Aborting.");
        return Ok(());
    }

    info!("System is now in WAIT_KEY state. Listening for USB events...");

    // 3. Main Event Loop (Simulation for now)
    loop {
        match state.current() {
            State::WaitKey => {
                if let Ok(Some(_key)) = usb_monitor::wait_for_key().await {
                    info!("Key detected! Verifying integrity...");
                    state.transition_to(State::Normal);
                }
            }
            State::Normal => {
                sleep(Duration::from_secs(5)).await;
                info!("Heartbeat: System NORMAL. Queue size: 0");
            }
            State::Freeze => {
                error!("System FROZEN. Waiting for Admin intervention via tuffctl...");
                sleep(Duration::from_secs(10)).await;
            }
            _ => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
