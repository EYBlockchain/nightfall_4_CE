use log::debug;

use warp::{path, Filter};

use crate::initialisation::get_block_assembly_status;

pub fn pause_block_assembly(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "pause")
        .and(warp::get())
        .and_then(handle_pause_block_assembly)
}

pub async fn handle_pause_block_assembly() -> Result<impl warp::Reply, warp::Rejection> {
    debug!("Block assembly is being paused");
    get_block_assembly_status().await.write().await.pause();
    Ok(warp::http::StatusCode::OK)
}
pub fn resume_block_assembly(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    path!("v1" / "resume")
        .and(warp::get())
        .and_then(handle_resume_block_assembly)
}
pub async fn handle_resume_block_assembly() -> Result<impl warp::Reply, warp::Rejection> {
    debug!("Block assembly is being resumed");
    get_block_assembly_status().await.write().await.resume();
    Ok(warp::http::StatusCode::OK)
}
