use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext};
use redis::AsyncCommands;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{api_error::ApiError, context::Context, puzzle::Puzzle};

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetPuzzleByIdPathParams {
    pub puzzle_id: Uuid,
}

/** Fetch a puzzle by uuid */
#[endpoint {
    method = GET,
    path = "/puzzle/{puzzleId}",
    tags = [ "puzzle" ],
}]
async fn get_puzzle_by_id(
    rqctx: Arc<RequestContext>,
    path_params: Path<GetPuzzleByIdPathParams>,
) -> Result<HttpResponseOk<Puzzle>, HttpError> {
    let ctx = Context::from_rqctx(&rqctx);
    let id = path_params.into_inner().puzzle_id;
    let puzzle: Puzzle = ctx
        .redis
        .clone()
        .hgetall(format!("puzzle:{}", id))
        .await
        .map_err(ApiError::Redis)?;

    Ok(HttpResponseOk(puzzle))
}