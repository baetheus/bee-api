use dropshot::HttpError;
use redis::RedisError;

#[derive(Debug)]
pub enum ApiError {
    Redis(RedisError),
}

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> Self {
        ApiError::Redis(error)
    }
}

impl From<ApiError> for HttpError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::Redis(error) => {
                HttpError::for_internal_error(format!("Unable to access datastore: {:?}", error))
            }
        }
    }
}
