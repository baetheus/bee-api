use std::sync::Arc;

use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
use serde_json::Value;

use crate::models::context::Context;

type Response<T> = Result<HttpResponseOk<T>, HttpError>;

/** Create a puzzle by uuid */
#[endpoint {
    method = GET,
    path = "/openapi",
    tags = [ "puzzle" ],
}]
pub async fn read_openapi(rqctx: Arc<RequestContext>) -> Response<Value> {
    Ok(HttpResponseOk(Context::from_rqctx(&rqctx).openapi.clone()))
}
