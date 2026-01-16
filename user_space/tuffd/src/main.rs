use anyhow::Result;
use log::{info, error};
use tokio::time::{sleep, Duration};

mod state_machine;
mod usb_monitor;
mod fs_manager;
mod events;

use state_machine::{SystemState, State};
use events::{TuffLogEntry, LogLevel, TuffEvent};
use tuff_common::schemas::{build_minimal_index_chunk, validate_index_chunk};

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
                        let fs = fs_manager::FsManager;
                        let mut promote_normal = true;
                        match fs.load_latest_index_chunk()? {
                            Some(buf) => {
                                if let Err(e) = validate_index_chunk(&buf) {
                                    error!("IndexChunk validation failed: {}", e);
                                    state.transition_to(State::Warn);
                                    promote_normal = false;
                                }
                            }
                            None => {
                                info!("IndexChunk not found yet; creating placeholder.");
                                let placeholder = build_minimal_index_chunk("tuff-volume", 1)?;
                                if let Err(e) = fs.write_latest_index_chunk(&placeholder) {
                                    error!("Failed to write placeholder IndexChunk: {}", e);
                                    state.transition_to(State::Warn);
                                    promote_normal = false;
                                } else if let Ok(Some(back)) = fs.load_latest_index_chunk() {
                                    if let Err(e) = validate_index_chunk(&back) {
                                        error!("Post-write IndexChunk validation failed: {}", e);
                                        state.transition_to(State::Warn);
                                        promote_normal = false;
                                    }
                                } else {
                                    error!("Post-write IndexChunk readback failed");
                                    state.transition_to(State::Warn);
                                    promote_normal = false;
                                }
                            }
                        }
                        if promote_normal {
                            state.transition_to(State::Normal);
                        }
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
