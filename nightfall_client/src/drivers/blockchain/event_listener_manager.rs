use crate::ports::contracts::NightfallContract;
use crate::ports::trees::CommitmentTree;
use crate::{
    drivers::blockchain::nightfall_event_listener::start_event_listener,
    initialisation::get_db_connection,
};

use lib::shared_entities::SynchronisationPhase::Synchronized;
use crate::drivers::blockchain::nightfall_event_listener::get_synchronisation_status;
use ark_bn254::Fr as Fr254;
use configuration::settings::get_settings;
use log::{debug, info, warn};
use mongodb::Client as MongoClient;
use tokio::{
    sync::{Mutex, OnceCell, RwLock},
    task::JoinHandle,
    time::{sleep, Duration},
};

// The sole place that holds the listener handle.
static LISTENER: OnceCell<RwLock<Option<JoinHandle<()>>>> = OnceCell::const_new();
// Add a restart lock to prevent concurrent restarts
static RESTART_LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();

async fn listener_lock() -> &'static RwLock<Option<JoinHandle<()>>> {
    // Tokio's OnceCell requires an async initializer.
    LISTENER.get_or_init(|| async { RwLock::new(None) }).await
}
async fn restart_lock() -> &'static Mutex<()> {
    RESTART_LOCK.get_or_init(|| async { Mutex::new(()) }).await
}

// Spawns the actual listener; logs errors; returns JoinHandle<()>.
async fn spawn_listener<N: NightfallContract>() -> JoinHandle<()> {
    let s = get_settings();
    let genesis = s.genesis_block;
    let max_attempts = s.nightfall_client.max_event_listener_attempts.unwrap_or(10);

    tokio::spawn(async move {
        let _ = start_event_listener::<N>(genesis, max_attempts).await; // discard Result
    })
}

/// Start once if not already running.
pub async fn ensure_running<N: NightfallContract>() {
    let lock = listener_lock().await;
    let mut guard = lock.write().await;
    if guard.is_none() {
        *guard = Some(spawn_listener::<N>().await);
        info!("Event listener started");
    }
}

/// Abort current (if any) and respawn.
pub async fn restart<N: NightfallContract>() {
    // Acquire restart lock to prevent concurrent restarts
    let _restart_guard = restart_lock().await.lock().await;

    let lock: &'static RwLock<Option<JoinHandle<()>>> = listener_lock().await;
    let mut guard = lock.write().await;

    if let Some(handle) = guard.take() {
        warn!("Restarting event listener: aborting current task…");
        handle.abort();
        // Wait for the task to actually terminate
        let _ = handle.await;
        // small grace to allow sockets/cursors to unwind
        sleep(Duration::from_millis(100)).await;
    } else {
        warn!("No event listener running to restart");
        return;
    }

    // Clean database and reset trees
    {
        let db = get_db_connection().await;
        let _ = <MongoClient as CommitmentTree<Fr254>>::reset_tree(db).await;
    }

    // Spawn new listener
    *guard = Some(spawn_listener::<N>().await);

    // Drop the listener guard to allow the listener to run
    drop(guard);

    // NOW WAIT FOR SYNCHRONIZATION WHILE HOLDING THE RESTART LOCK
    let max_wait_time = Duration::from_secs(300); // 5 minutes max
    let start_time = std::time::Instant::now();

    loop {
        sleep(Duration::from_secs(5)).await; // Check every 5 seconds
        debug!("Checking synchronization status...");
        match get_synchronisation_status::<N>().await {
            Ok(status) => {
                if status.phase() == Synchronized {
                    info!("Event listener synchronized successfully");
                    break;
                } else {
                    debug!("Still synchronizing... ({:?})", status.phase());
                }
            }
            Err(e) => {
                warn!("Error checking sync status during restart: {e:?}",);
            }
        }

        // Timeout check
        if start_time.elapsed() > max_wait_time {
            warn!(
                "Restart timeout - releasing lock after {} seconds",
                max_wait_time.as_secs()
            );
            break;
        }
    }

    debug!("Restart lock released after synchronization check");
    // Lock is automatically released when _restart_guard goes out of scope
}
