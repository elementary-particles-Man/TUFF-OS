use anyhow::Result;
use log::{info, error};
use tokio::time::{sleep, Duration};

mod state_machine;
mod usb_monitor;
mod fs_manager;
mod events;

use state_machine::{SystemState, State};
use events::{TuffLogEntry, LogLevel, TuffEvent};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    TuffLogEntry::new(
        LogLevel::Info,
        TuffEvent::SystemBoot { version: env!("CARGO_PKG_VERSION").to_string() },
    ).log();

    // 1. Initialize State Machine
    let mut state = SystemState::new();

    // 2. Transition to WAIT_KEY
    if state.current() == State::Init {
        state.transition_to(State::WaitKey);
        TuffLogEntry::new(
            LogLevel::Info,
            TuffEvent::StateTransition {
                from: State::Init,
                to: State::WaitKey,
                reason: "Boot sequence".into(),
            },
        ).log();
    }

    info!("System is now in WAIT_KEY state. Listening for USB events...");

    // 3. Main Event Loop
    loop {
        match state.current() {
            State::WaitKey => {
                match usb_monitor::wait_for_key().await {
                    Ok(Some((_key, uuid))) => {
                        info!("Key {} accepted.", uuid);
                        state.transition_to(State::Normal);
                        TuffLogEntry::new(
                            LogLevel::Info,
                            TuffEvent::StateTransition {
                                from: State::WaitKey,
                                to: State::Normal,
                                reason: "Key authenticated".into(),
                            },
                        ).log();
                    }
                    Err(e) => {
                        error!("USB Monitor error: {}", e);
                        sleep(Duration::from_secs(5)).await;
                    }
                    Ok(None) => {
                        // Should not happen with current wait_for_key impl as it loops internally
                    }
                }
            }
            State::Normal => {
                sleep(Duration::from_secs(10)).await;
            }
            State::Freeze => {
                error!("System FROZEN. Waiting for Admin intervention.");
                sleep(Duration::from_secs(10)).await;
            }
            _ => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
