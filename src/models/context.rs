use std::{any::Any, sync::Arc};

use dropshot::RequestContext;
use redis::aio::MultiplexedConnection;

pub struct Context {
    pub redis: MultiplexedConnection,
}

impl Context {
    pub async fn new(redis_address: &str) -> Arc<Context> {
        let redis = redis::Client::open(redis_address.to_owned())
            .expect(&format!(
                "Unable to open client connection to {}",
                &redis_address
            ))
            .create_multiplexed_tokio_connection()
            .await
            .expect("Could not create multiplexed redis connection.")
            .0;

        Arc::new(Context { redis: redis })
    }

    pub fn from_rqctx(rqctx: &Arc<RequestContext>) -> Arc<Context> {
        let ctx: Arc<dyn Any + Send + Sync + 'static> = Arc::clone(&rqctx.server.private);
        ctx.downcast::<Context>()
            .expect("wrong type for private data")
    }
}
