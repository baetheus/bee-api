use std::{any::Any, sync::Arc};

use dropshot::{OpenApiDefinition, RequestContext};
use redis::aio::MultiplexedConnection;
use serde_json::Value;

pub struct Context {
    pub redis: MultiplexedConnection,
    pub openapi: Value,
}

impl Context {
    pub async fn new<'a>(redis_address: &str, openapi: OpenApiDefinition<'a>) -> Arc<Context> {
        let redis = redis::Client::open(redis_address.to_owned())
            .expect(&format!(
                "Unable to open client connection to {}",
                &redis_address
            ))
            .get_multiplexed_tokio_connection()
            .await
            .expect("Could not create multiplexed redis connection.");

        Arc::new(Context {
            redis: redis,
            openapi: openapi
                .json()
                .expect("Unable to convert openapi definition to json"),
        })
    }

    pub fn from_rqctx(rqctx: &Arc<RequestContext>) -> Arc<Context> {
        let ctx: Arc<dyn Any + Send + Sync + 'static> = Arc::clone(&rqctx.server.private);
        ctx.downcast::<Context>()
            .expect("wrong type for private data")
    }
}
