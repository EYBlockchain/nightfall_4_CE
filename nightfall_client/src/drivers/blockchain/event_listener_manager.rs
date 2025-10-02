use crate::ports::trees::CommitmentTree;
use crate::{drivers::blockchain::nightfall_event_listener::start_event_listener, initialisation::get_db_connection};
use crate::ports::contracts::NightfallContract;

use configuration::settings::get_settings;
use log::{info, warn};
use tokio::{
    sync::{OnceCell, RwLock},
    task::JoinHandle,
    time::{sleep, Duration},
};
use mongodb::Client as MongoClient;
use crate::domain::entities::SynchronisationPhase::Synchronized;
use crate::drivers::blockchain::nightfall_event_listener::get_synchronisation_status;
use ark_bn254::Fr as Fr254;

// The sole place that holds the listener handle.
static LISTENER: OnceCell<RwLock<Option<JoinHandle<()>>>> = OnceCell::const_new();
async fn listener_lock() -> &'static RwLock<Option<JoinHandle<()>>> {
    // Tokio's OnceCell requires an async initializer.
    LISTENER.get_or_init(|| async { RwLock::new(None) }).await
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
        info!("Event listener started.");
    }
}

/// Abort current (if any) and respawn.
pub async fn restart<N: NightfallContract>() {
    let lock = listener_lock().await;
    let mut guard = lock.write().await;

    if let Some(handle) = guard.take() {
        warn!("Restarting event listener: aborting current task…");
        handle.abort();
        // small grace to allow sockets/cursors to unwind
        sleep(Duration::from_millis(50)).await;
    }

    // if we're restarting the event lister, we definitely shouldn't be in sync, so check that's the case
    let sync_state = get_synchronisation_status::<N>()
        .await
        .expect("Could not check synchronisation state")
        .phase();
    if sync_state == Synchronized {
        panic!("Restarting event listener while synchronised. This should not happen");
    }
    let settings = get_settings();
    let max_attempts = settings
        .nightfall_client
        .max_event_listener_attempts
        .unwrap_or(10);

    // clean the database and reset the trees
    // this is a bit of a hack, but we need to reset the trees to get them back in sync
    // with the blockchain. We should probably do this in a more elegant way, but this works for now
    // and we can improve it later
    {
        let db = get_db_connection().await;
        let _ = <MongoClient as CommitmentTree<Fr254>>::reset_tree(db).await;
    }

    *guard = Some(spawn_listener::<N>().await);
    info!("Event listener restarted.");
}
