use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};
use uuid::Uuid;

use redis::{ErrorKind, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PartialUser {
    pub name: String,
    pub email: String,
    pub admin: bool,
}

impl FromRedisValue for PartialUser {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(v) => {
                let slice: &[u8] = v;
                let result: PartialUser =
                    from_slice(slice).map_err(|_| (ErrorKind::TypeError, "Parsing Error"))?;
                Ok(result)
            }
            _ => Err((
                ErrorKind::TypeError,
                "Incorrect redis value for PartialUser",
            ))?,
        }
    }
}

impl ToRedisArgs for PartialUser {
    fn write_redis_args<W: ?Sized>(&self, out: &mut W)
    where
        W: RedisWrite,
    {
        let output: &[u8] = &to_vec(self).expect("Incorrect struct for PartialUser");
        out.write_arg(output);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FromRedisValue for User {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(v) => {
                let slice: &[u8] = v;
                let result: User =
                    from_slice(slice).map_err(|_| (ErrorKind::TypeError, "Parsing Error"))?;
                Ok(result)
            }
            _ => Err((ErrorKind::TypeError, "Incorrect redis value for User"))?,
        }
    }
}

impl ToRedisArgs for User {
    fn write_redis_args<W: ?Sized>(&self, out: &mut W)
    where
        W: RedisWrite,
    {
        let output: &[u8] = &to_vec(self).expect("Incorrect struct for User");
        out.write_arg(output);
    }
}
