use anyhow::Result;
use log::{info, error};
use nix::mount::{mount, MsFlags};
use nix::errno::Errno;
use nix::unistd::getpid;
use tokio::time::{sleep, Duration};

mod state_machine;
mod usb_monitor;
mod fs_manager;
mod events;
mod mk_fingerprint;

use state_machine::{SystemState, State};
use events::{TuffLogEntry, LogLevel, TuffEvent};
use tuff_common::schemas::{build_minimal_index_chunk, validate_index_chunk};
use mk_fingerprint::{verify_or_store_mk_fingerprint, FingerprintStatus};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    if is_pid1() {
        if let Err(e) = early_boot_setup() {
            error!("Early boot setup failed: {}", e);
        }
    }

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
                    Ok(Some((key, uuid))) => {
                        info!("Key {} accepted.", uuid);
                        match verify_or_store_mk_fingerprint(&key) {
                            Ok(FingerprintStatus::Matched) | Ok(FingerprintStatus::Stored) => {}
                            Ok(FingerprintStatus::Mismatch) => {
                                state.transition_to(State::Freeze);
                                TuffLogEntry::new(
                                    LogLevel::Error,
                                    TuffEvent::KeyMismatch {
                                        reason: "MK fingerprint mismatch; refusing to proceed".into(),
                                    },
                                ).log();
                                TuffLogEntry::new(
                                    LogLevel::Warn,
                                    TuffEvent::StateTransition {
                                        from: State::WaitKey,
                                        to: State::Freeze,
                                        reason: "MK fingerprint mismatch".into(),
                                    },
                                ).log();
                                sleep(Duration::from_secs(10)).await;
                                continue;
                            }
                            Err(e) => {
                                state.transition_to(State::Warn);
                                TuffLogEntry::new(
                                    LogLevel::Error,
                                    TuffEvent::IoError {
                                        context: "MK fingerprint check failed".into(),
                                        error: e.to_string(),
                                    },
                                ).log();
                                sleep(Duration::from_secs(5)).await;
                                continue;
                            }
                        }
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

fn is_pid1() -> bool {
    getpid().as_raw() == 1
}

fn early_boot_setup() -> Result<()> {
    use std::fs;
    use std::path::Path;

    let dirs = ["/proc", "/sys", "/dev", "/dev/pts"];
    for dir in dirs.iter() {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir)?;
        }
    }

    mount_once(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV,
    )?;
    mount_once(
        Some("sysfs"),
        "/sys",
        Some("sysfs"),
        MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV,
    )?;
    mount_once(
        Some("devtmpfs"),
        "/dev",
        Some("devtmpfs"),
        MsFlags::MS_NOSUID,
    )?;
    mount_once(
        Some("devpts"),
        "/dev/pts",
        Some("devpts"),
        MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC,
    )?;

    Ok(())
}

fn mount_once(
    source: Option<&str>,
    target: &str,
    fstype: Option<&str>,
    flags: MsFlags,
) -> Result<()> {
    match mount(source, target, fstype, flags, None::<&str>) {
        Ok(()) => Ok(()),
        Err(Errno::EBUSY) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
