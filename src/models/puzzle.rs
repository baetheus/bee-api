use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};
use uuid::Uuid;

use redis::{ErrorKind, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct PartialPuzzle {
    pub name: String,
    pub released_at: Option<DateTime<Utc>>,
    pub words: Vec<String>,
    pub letters: Vec<String>,
    pub middle: String,
    pub language: String,
}

impl FromRedisValue for PartialPuzzle {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(v) => {
                let slice: &[u8] = v;
                let result: PartialPuzzle =
                    from_slice(slice).map_err(|_| (ErrorKind::TypeError, "Parsing Error"))?;
                Ok(result)
            }
            _ => Err((
                ErrorKind::TypeError,
                "Incorrect redis value for PartialPuzzle",
            ))?,
        }
    }
}

impl ToRedisArgs for PartialPuzzle {
    fn write_redis_args<W: ?Sized>(&self, out: &mut W)
    where
        W: RedisWrite,
    {
        let output: &[u8] = &to_vec(self).expect("Incorrect struct for PartialPuzzle");
        out.write_arg(output);
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct Puzzle {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub words: Vec<String>,
    pub letters: Vec<String>,
    pub middle: String,
    pub language: String,
}

impl FromRedisValue for Puzzle {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(v) => {
                let slice: &[u8] = v;
                let result: Puzzle =
                    from_slice(slice).map_err(|_| (ErrorKind::TypeError, "Parsing Error"))?;
                Ok(result)
            }
            _ => Err((ErrorKind::TypeError, "Incorrect redis value for Puzzle"))?,
        }
    }
}

impl ToRedisArgs for Puzzle {
    fn write_redis_args<W: ?Sized>(&self, out: &mut W)
    where
        W: RedisWrite,
    {
        let output: &[u8] = &to_vec(self).expect("Incorrect struct for Puzzle");
        out.write_arg(output);
    }
}
