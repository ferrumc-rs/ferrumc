use axum::extract::State;
use axum::response::Json;
use ferrumc_state::{GlobalState, Stats};
use std::sync::Arc;

pub(crate) async fn stats_handler(
    State(state): State<GlobalState>,
) -> Json<Arc<Stats>> {
    Json(state.stats.clone())
}