use dropshot::{endpoint, HttpError, HttpResponseOk, Path, RequestContext, TypedBody};
use redis::AsyncCommands;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{
    api_error::ApiError,
    context::Context,
    puzzle::{PartialPuzzle, Puzzle},
};

type Response<T> = Result<HttpResponseOk<T>, HttpError>;

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetPuzzleByIdPathParams {
    pub puzzle_id: Uuid,
}

/** Create a puzzle by uuid */
#[endpoint {
    method = POST,
    path = "/puzzle",
    tags = [ "puzzle" ],
}]
pub async fn create_puzzle(
    rqctx: Arc<RequestContext>,
    body: TypedBody<PartialPuzzle>,
) -> Response<Puzzle> {
    let mut redis = Context::from_rqctx(&rqctx).redis.clone();
    let puzzle: Puzzle = body.into_inner().into();

    redis
        .set(puzzle.to_key(), puzzle.to_owned())
        .await
        .map_err(ApiError::Redis)?;

    Ok(HttpResponseOk(puzzle))
}

/** Read a puzzle by uuid */
#[endpoint {
    method = GET,
    path = "/puzzle/{puzzleId}",
    tags = [ "puzzle" ],
}]
pub async fn read_puzzle(
    rqctx: Arc<RequestContext>,
    path_params: Path<GetPuzzleByIdPathParams>,
) -> Response<Puzzle> {
    let mut redis = Context::from_rqctx(&rqctx).redis.clone();
    let key = format!("puzzle:{}", path_params.into_inner().puzzle_id);

    let has_id: bool = redis.exists(&key).await.map_err(ApiError::Redis)?;
    if !has_id {
        return Err(HttpError::for_not_found(
            Some("404".to_owned()),
            format!("Key not found {}", key).to_owned(),
        ));
    }

    let puzzle: Puzzle = redis.get(key).await.map_err(ApiError::Redis)?;

    Ok(HttpResponseOk(puzzle))
}

/** Update a puzzle by uuid */
#[endpoint {
    method = PUT,
    path = "/puzzle/{puzzleId}",
    tags = [ "puzzle" ],
}]
pub async fn update_puzzle(
    rqctx: Arc<RequestContext>,
    path_params: Path<GetPuzzleByIdPathParams>,
    body: TypedBody<Puzzle>,
) -> Response<Puzzle> {
    let mut redis = Context::from_rqctx(&rqctx).redis.clone();
    let puzzle_id = path_params.into_inner().puzzle_id;
    let puzzle = body.into_inner();

    if puzzle.id != puzzle_id {
        return Err(HttpError::for_bad_request(
            Some("400".to_owned()),
            format!(
                "Path id ({}) does not match puzzle id ({})",
                puzzle_id, puzzle.id
            ),
        ));
    }

    let key = format!("puzzle:{}", puzzle_id);

    let has_id: bool = redis.exists(&key).await.map_err(ApiError::Redis)?;
    if !has_id {
        return Err(HttpError::for_not_found(
            Some("404".to_owned()),
            format!("Key not found {}", key).to_owned(),
        ));
    }

    redis
        .set(puzzle.to_key(), puzzle.to_owned())
        .await
        .map_err(ApiError::Redis)?;

    Ok(HttpResponseOk(puzzle))
}

/** Read a puzzle by uuid */
#[endpoint {
    method = DELETE,
    path = "/puzzle/{puzzleId}",
    tags = [ "puzzle" ],
}]
pub async fn delete_puzzle(
    rqctx: Arc<RequestContext>,
    path_params: Path<GetPuzzleByIdPathParams>,
) -> Response<()> {
    let mut redis = Context::from_rqctx(&rqctx).redis.clone();
    let key = format!("puzzle:{}", path_params.into_inner().puzzle_id);

    let has_id: bool = redis.exists(&key).await.map_err(ApiError::Redis)?;
    if !has_id {
        return Err(HttpError::for_not_found(
            Some("404".to_owned()),
            format!("Key not found {}", key).to_owned(),
        ));
    }

    redis.del(key).await.map_err(ApiError::Redis)?;

    Ok(HttpResponseOk(()))
}

/** Read all puzzlese */
#[endpoint {
    method = GET,
    path = "/puzzle",
    tags = [ "puzzle" ],
}]
pub async fn read_puzzles(rqctx: Arc<RequestContext>) -> Response<Vec<Puzzle>> {
    let mut redis = Context::from_rqctx(&rqctx).redis.clone();

    let keys: Vec<String> = redis.keys("puzzle:*").await.map_err(ApiError::Redis)?;
    let keys: Vec<String> = keys.into_iter().collect();

    match &keys.len() {
        0 => {
            let puzzles: Vec<Puzzle> = vec![];
            Ok(HttpResponseOk(puzzles))
        }
        1 => {
            let puzzle: Puzzle = redis.get(keys).await.map_err(ApiError::Redis)?;
            let puzzles = vec![puzzle];
            Ok(HttpResponseOk(puzzles))
        }
        _ => {
            let puzzles: Vec<Puzzle> = redis.get(keys).await.map_err(ApiError::Redis)?;
            Ok(HttpResponseOk(puzzles))
        }
    }
}
