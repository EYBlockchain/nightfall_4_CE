use crate::drivers::blockchain::nightfall_event_listener::start_event_listener;
use crate::initialisation::get_runtime_listener_start_block;
use crate::ports::contracts::NightfallContract;
use configuration::settings::get_settings;
use futures::FutureExt;
use lib::nf_client_proof::{Proof, ProvingEngine};
use log::{info, warn};
use std::panic::AssertUnwindSafe;
use tokio::{
    sync::{OnceCell, RwLock},
    task::JoinHandle,
    time::{sleep, Duration},
};

// The sole place that holds the listener handle.
static LISTENER: OnceCell<RwLock<Option<JoinHandle<()>>>> = OnceCell::const_new();
async fn listener_lock() -> &'static RwLock<Option<JoinHandle<()>>> {
    // Tokio's OnceCell requires an async initializer.
    LISTENER.get_or_init(|| async { RwLock::new(None) }).await
}

// Spawns the actual listener; logs errors; returns JoinHandle<()>.
async fn spawn_listener<P, E, N>() -> JoinHandle<()>
where
    P: Proof,
    E: ProvingEngine<P>,
    N: NightfallContract,
{
    let s = get_settings();
    let start_block = get_runtime_listener_start_block().await;
    let max_attempts = s
        .nightfall_proposer
        .max_event_listener_attempts
        .unwrap_or(10);

    tokio::spawn(async move {
        let mut next_start_block = start_block;
        loop {
            match AssertUnwindSafe(start_event_listener::<P, E, N>(
                next_start_block,
                max_attempts,
            ))
            .catch_unwind()
            .await
            {
                Ok(()) => warn!(
                    "Event listener task exited after exhausting its retry budget; restarting supervisor loop shortly."
                ),
                Err(_) => warn!(
                    "Event listener task panicked; restarting supervisor loop shortly."
                ),
            }
            next_start_block = get_runtime_listener_start_block().await;
            sleep(Duration::from_secs(1)).await;
        }
    })
}

/// Start once if not already running.
pub async fn ensure_running<P: Proof, E: ProvingEngine<P>, N: NightfallContract>() {
    let lock = listener_lock().await;
    let mut guard = lock.write().await;
    let should_spawn = match guard.as_ref() {
        None => true,
        Some(handle) if handle.is_finished() => {
            warn!("Event listener task had already stopped; spawning a replacement.");
            true
        }
        Some(_) => false,
    };

    if should_spawn {
        *guard = Some(spawn_listener::<P, E, N>().await);
        info!("Event listener started.");
    }
}

/// Abort current (if any) and respawn.
pub async fn restart<P: Proof, E: ProvingEngine<P>, N: NightfallContract>() {
    let lock = listener_lock().await;
    let mut guard = lock.write().await;

    if let Some(handle) = guard.take() {
        warn!("Restarting event listener: aborting current task…");
        handle.abort();
        // small grace to allow sockets/cursors to unwind
        sleep(Duration::from_millis(50)).await;
    }

    *guard = Some(spawn_listener::<P, E, N>().await);
    info!("Event listener restarted.");
}
